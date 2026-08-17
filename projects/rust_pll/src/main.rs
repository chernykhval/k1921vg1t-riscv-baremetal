#![no_std]
#![no_main]

use core::panic::PanicInfo;
use k1921vg1t_pac as pac;
use riscv_rt::entry;

// ============================================================================
//  ЦЕРЕМОНИЯ №1 — UART (тактируется от HSE, поэтому PLL на него не влияет:
//  вывод продолжает работать на 9600 даже после разгона ядра).
// ============================================================================
fn uart0_init(p: &pac::Peripherals) {
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
//  ИЗМЕРЕНИЕ SYSCLK — TMR0 считает такты ядра, а эталон времени задаёт
//  передача N байт по UART (UART на точном HSE). Как ловили HSI.
// ============================================================================
fn tmr0_counter_start(p: &pac::Peripherals) {
    p.rcu.cgcfgapb1().modify(|_, w| w.tmr0en().set_bit());
    p.rcu.rstdisapb1().modify(|_, w| w.tmr0en().set_bit());
    p.tmr0.period().write(|w| unsafe { w.val().bits(0xFFFF_FFFF) });
    p.tmr0.ctrl().modify(|_, w| w.mode().up());
    p.siu.tmren().modify(|_, w| w.tmr0en().set_bit());
}

fn measure_sysclk(p: &pac::Peripherals) -> u32 {
    const N: u32 = 2000;
    let t0 = p.tmr0.count().read().val().bits();
    for _ in 0..N {
        uart0_send_byte(p, b'.');
    }
    while p.uart0.fr().read().busy().bit_is_set() {}
    let t1 = p.tmr0.count().read().val().bits();
    let ticks = t1.wrapping_sub(t0) as u64;
    (ticks * 9600 / (N as u64 * 10)) as u32
}

// ============================================================================
//  ЦЕРЕМОНИЯ №2 — РАЗГОН через PLL0. Рецепт НИИЭТ: 27 МГц HSE → 204 МГц.
//  ОПАСНО: неверная настройка -> зависание. Порядок важен.
// ============================================================================
fn pll_init(p: &pac::Peripherals) {
    // 1. кварц HSE (источник для PLL) должен работать
    p.rcu.hsecfg().modify(|_, w| w.en().set_bit());
    for _ in 0..100_000 {
        unsafe { core::arch::asm!("nop") };
    }

    // 2. коэффициенты PLL0 (делители/множитель) — рецепт НИИЭТ для 27 МГц
    unsafe {
        p.rcu.pll(0).div().write_with_zero(|w| {
            w.div1a().bits(1);
            w.div1b().bits(2);
            w.prediv().bits(1);
            w.nnclr().set_bit(); // включить N-делитель
            w.rnclr().set_bit(); // включить R-делитель
            w.rdiv().bits(2);
            w.ndiv().bits(100) // множитель
        });
    }
    p.rcu.pll(0).mod_().write(|w| unsafe { w.mod_().bits(1) });
    p.rcu.pll(0).frac().write(|w| unsafe { w.frac().bits(1) });
    unsafe {
        p.rcu.pll(0).cfg().write_with_zero(|w| {
            w.fouten().set_bit(); // разрешить выход
            w.pfd().bits(3); // рабочий режим ЧФД
            w.clksel().clear_bit(); // 0 = источник HSE
            w.vcomode().bits(1);
            w.cp().bits(9);
            w.st().bits(0) // целочисленный делитель
        });
    }

    // 3. дать PLL время на захват частоты (обычно << мс; ~30 мс с запасом на HSI)
    for _ in 0..100_000 {
        unsafe { core::arch::asm!("nop") };
    }

    // 4. ВАЖНО: увеличить задержку доступа к Flash — на 204 МГц флеш «не успевает»
    //    за ядром, без этого чтение инструкций сломается и чип зависнет.
    p.flashm.ctrl().modify(|_, w| unsafe { w.lat().bits(2) });

    // 5. переключить системный такт на PLL0 (src = 2)
    p.rcu.sysclkcfg().write(|w| unsafe { w.src().bits(2) });
    while p.rcu.clkstat().read().src().bits() != 2 {}
}

#[entry]
fn main() -> ! {
    let p = unsafe { pac::Peripherals::steal() };
    uart0_init(&p);
    tmr0_counter_start(&p);

    // измерить частоту ДО разгона (сейчас ядро на HSI ~3.4 МГц)
    uart0_send_str(&p, "\r\nBefore PLL: ");
    let before = measure_sysclk(&p);
    uart0_send_u32(&p, before);
    uart0_send_str(&p, " Hz\r\n");

      // РАЗОГНАТЬ через PLL (27 -> 204 МГц)
    pll_init(&p);

      // измерить ПОСЛЕ
    uart0_send_str(&p, "After PLL:  ");
    let after = measure_sysclk(&p);
    uart0_send_u32(&p, after);
    uart0_send_str(&p, " Hz\r\n");

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
