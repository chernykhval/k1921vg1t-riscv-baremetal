#![no_std]
#![no_main]

use core::panic::PanicInfo;
use k1921vg1t_pac as pac;
use riscv_rt::entry;

fn uart0_init(p: &pac::Peripherals) {
    // UART-такт берём от кварца HSE (27 МГц) — он ТОЧНЫЙ, в отличие от
    // RC-генератора HSI (гуляет на проценты → мусор в UART).
    // Включаем осциллятор HSE и даём кварцу время на раскачку (~мс).
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
        w.clksel().srcclk();   // srcclk = HSE (кварц 27 МГц), точный такт
        w.clken().set_bit();
        w.rstdis().set_bit()
    });

    // baud 9600 от HSE = 27 МГц:
    // делитель = 27_000_000 / (16 * 9600) = 175.78 → IBRD=175, FBRD=round(0.78*64)=50. Ровно 9600.
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

// Печать числа в десятичном виде (в UART нет printf — делаем сами).
fn uart0_send_u32(p: &pac::Peripherals, mut n: u32) {
    if n == 0 {
        uart0_send_byte(p, b'0');
        return;
    }
    let mut buf = [0u8; 10]; // u32 <= 4_294_967_295, максимум 10 цифр
    let mut i = 0;
    while n > 0 {
        buf[i] = b'0' + (n % 10) as u8; // младшая цифра
        n /= 10;
        i += 1;
    }
    // цифры получились в обратном порядке — шлём с конца
    while i > 0 {
        i -= 1;
        uart0_send_byte(p, buf[i]);
    }
}

// Запуск TMR0 как свободного счётчика тактов ЯДРА (HSI). Без прерываний —
// просто считает вверх, а мы читаем его значение через count().
fn tmr0_counter_start(p: &pac::Peripherals) {
    p.rcu.cgcfgapb1().modify(|_, w| w.tmr0en().set_bit());
    p.rcu.rstdisapb1().modify(|_, w| w.tmr0en().set_bit());
    p.tmr0.period().write(|w| unsafe { w.val().bits(0xFFFF_FFFF) }); // макс период — не переполнится
    p.tmr0.ctrl().modify(|_, w| w.mode().up());
    p.siu.tmren().modify(|_, w| w.tmr0en().set_bit());
}

#[entry]
fn main() -> ! {
    let p = unsafe { pac::Peripherals::steal() };

    uart0_init(&p);          // UART на точном кварце HSE
    tmr0_counter_start(&p);  // счётчик тактов ядра (HSI)

    uart0_send_str(&p, "\r\nHello from K1921VG1T!\r\n");
    uart0_send_str(&p, "Measuring HSI (wait ~2s)...\r\n");

    // ИЗМЕРЕНИЕ HSI:
    // ядро тикает от HSI (неточный), а передача по UART идёт от HSE (точный).
    // Передача N байт на 9600 бод длится ровно N*10/9600 секунд (10 бит на байт).
    // Считаем, сколько HSI-тиков уложилось в это эталонное время.
    const N: u32 = 2000;
    let t0 = p.tmr0.count().read().val().bits();
    for _ in 0..N {
        uart0_send_byte(&p, b'.'); // полезная нагрузка не важна — важно ВРЕМЯ передачи
    }
    while p.uart0.fr().read().busy().bit_is_set() {} // дождаться, пока последний байт реально ушёл
    let t1 = p.tmr0.count().read().val().bits();

    // HSI = тики / время = тики * 9600 / (N * 10). Считаем в u64, чтобы не переполнить.
    let ticks = t1.wrapping_sub(t0) as u64;
    let hsi = (ticks * 9600 / (N as u64 * 10)) as u32;

    uart0_send_str(&p, "\r\nHSI = ");
    uart0_send_u32(&p, hsi);
    uart0_send_str(&p, " Hz\r\n");

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
