#![no_std]
#![no_main]

use core::panic::PanicInfo;
use k1921vg1t_pac as pac;
use riscv_rt::entry;

// ============================================================================
//  ЦЕРЕМОНИЯ №1 — UART (готовый, из проекта rust_uart). Не трогаем.
//  Нужен, чтобы печатать измерения в терминал (COM5, 9600 8-N-1).
// ============================================================================
fn uart0_init(p: &pac::Peripherals) {
    // такт от кварца HSE (27 МГц) — точный
    p.rcu.hsecfg().modify(|_, w| w.en().set_bit());
    for _ in 0..100_000 {
        unsafe { core::arch::asm!("nop") };
    }
    p.rcu.cgcfgahb().modify(|_, w| w.gpioben().set_bit());
    p.rcu.rstdisahb().modify(|_, w| w.gpioben().set_bit());
    p.rcu.cgcfgapb2().modify(|_, w| w.uart0en().set_bit());
    p.rcu.rstdisapb2().modify(|_, w| w.uart0en().set_bit());
    p.gpiob.altfuncnum0().modify(|_, w| w.pin0().af3().pin1().af3());
    p.gpiob.altfuncset().write(|w| w.pin0().set_bit().pin1().set_bit());
    p.rcu.uartcfg(0).value().modify(|_, w| {
        w.clksel().srcclk();
        w.clken().set_bit();
        w.rstdis().set_bit()
    });
    p.uart0.ibrd().write(|w| unsafe { w.divint().bits(175) });
    p.uart0.fbrd().write(|w| unsafe { w.divfrac().bits(50) });
    p.uart0.lcrh().modify(|_, w| w.wlen()._8bit().fen().set_bit());
    p.uart0.cr().modify(|_, w| w.txe().set_bit().rxe().set_bit().uarten().set_bit());
}

fn uart0_send_byte(p: &pac::Peripherals, byte: u8) {
    while p.uart0.fr().read().txff().bit_is_set() {}
    p.uart0.dr().write(|w| unsafe { w.data().bits(byte) });
}

fn uart0_send_str(p: &pac::Peripherals, s: &str) {
    for b in s.bytes() {
        uart0_send_byte(p, b);
    }
}

// печать числа в десятичном виде (printf у нас нет)
fn uart0_send_u32(p: &pac::Peripherals, mut n: u32) {
    if n == 0 {
        uart0_send_byte(p, b'0');
        return;
    }
    let mut buf = [0u8; 10];
    let mut i = 0;
    while n > 0 {
        buf[i] = b'0' + (n % 10) as u8;
        n /= 10;
        i += 1;
    }
    while i > 0 {
        i -= 1;
        uart0_send_byte(p, buf[i]);
    }
}

// ============================================================================
//  ЦЕРЕМОНИЯ №2 — АЦП + датчик температуры (сложная аналоговая обвязка).
//  TSENSOR (датчик температуры кристалла) внутри подключён к каналу 63
//  на аналоговом блоке ADC3. Настраиваем: такт, питание фронтенда, секвенсор.
// ============================================================================
const TSENSOR_CH: u8 = 63; // канал АЦП, к которому подключён датчик температуры

fn adc_init(p: &pac::Peripherals) {
    // --- такт АЦП: источник HSE (тот же кварц, что у UART), включить блок ---
    p.rcu.adccfg().modify(|_, w| unsafe {
        w.clksel().bits(1); // 1 = HSE
        w.diven().clear_bit();
        w.clken().set_bit();
        w.rstdis().set_bit()
    });
    p.rcu.cgcfgahb().modify(|_, w| w.adcen().set_bit());
    p.rcu.rstdisahb().modify(|_, w| w.adcen().set_bit());

    // --- аналоговый фронтенд блока ADC3 (LDO-питание, разрешение 12 бит) ---
    p.adc.actl(3).ctrl().modify(|_, w| {
        w.adcenldo().set_bit(); // включить внутренний стабилизатор питания
        w.rangeldo().set_bit();
        w.dislvl().clear_bit();
        w.selres()._12bit(); // разрешение 12 бит (0..4095)
        w.calen().clear_bit(); // без калибровки
        w.adcen().set_bit() // включить сам АЦП
    });
    for _ in 0..500 {
        unsafe { core::arch::asm!("nop") }; // ~20 мкс на раскачку аналога
    }
    p.adc.actl(3).ctrl().modify(|_, w| w.adcstart().set_bit());

    // --- включить датчик температуры (SCALE влияет на формулу перевода!) ---
    p.siu.tsensctrl().modify(|_, w| {
        w.scale().set_bit();
        w.dis().clear_bit()
    });

    // --- секвенсор 0: одно измерение канала 63 по программному запросу ---
    p.adc.emux().modify(|_, w| unsafe { w.em0().bits(0) }); // 0 = запуск программой
    p.adc.seq(0).srqsel0().modify(|_, w| unsafe { w.rq0().bits(TSENSOR_CH) });
    p.adc.seq(0).srqctl().modify(|_, w| unsafe {
        w.rqmax().bits(0); // очередь из 1 запроса
        w.qavgen().clear_bit()
    });
    // доп. задержка на канале — датчику температуры нужно время «подключиться»
    p.adc.chdelay(TSENSOR_CH as usize).value().write(|w| unsafe { w.delay().bits(2700) });

    // --- включить секвенсор 0 ---
    p.adc.seqsync().modify(|_, w| w.sync0().set_bit());
    p.adc.seqen().modify(|_, w| w.seqen0().set_bit());

    // --- дождаться, пока АЦП завершит внутреннюю инициализацию ---
    while p.adc.actl(3).ctrl().read().adcrdy().bit_is_clear() {}
}

// Запустить одно измерение TSENSOR и вернуть сырое значение АЦП (0..4095).
fn adc_read_tsensor(p: &pac::Peripherals) -> u16 {
    // запустить одно преобразование (GSYNC запускает секвенсоры с установленным SYNC)
    p.adc.seqsync().modify(|_, w| w.gsync().set_bit());
    // ждать, пока в FIFO секвенсора появится результат (SFLOAD = число слов в буфере)
    while p.adc.seq(0).sfload().read().val().bits() == 0 {}
    // прочитать результат — чтение опустошает ячейку FIFO
    p.adc.seq(0).sfifo().read().data().bits()
}

fn raw_to_temp_x10(raw: u16) -> i32 {
    let v_mv = raw as i32 * 3300 / 4096;   // напряжение датчика в милливольтах
    574713 * (793 - v_mv) / 100000         // температура в десятых долях градуса
}

#[entry]
fn main() -> ! {
    let p = unsafe { pac::Peripherals::steal() };
    uart0_init(&p);
    adc_init(&p);
    
    loop {
        let raw = adc_read_tsensor(&p);
        let t10 = raw_to_temp_x10(raw);        // температура ×10

        uart0_send_str(&p, "T = ");
        uart0_send_u32(&p, (t10 / 10) as u32); // целые градусы
        uart0_send_str(&p, ".");
        uart0_send_u32(&p, (t10 % 10) as u32); // десятая доля
        uart0_send_str(&p, " C\r\n");

        for _ in 0..3_000_000 { unsafe { core::arch::asm!("nop") } }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
