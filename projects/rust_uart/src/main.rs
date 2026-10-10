#![no_std]
#![no_main]

use core::panic::PanicInfo;
use k1921vg1t_pac as pac;
use riscv_rt::entry;

fn uart0_init(p: &pac::Peripherals) {
    // UART0 is clocked from the 27 MHz HSE crystal: the internal RC oscillator (HSI)
    // is too inaccurate for a UART. Enable HSE and give the crystal time to start.
    p.rcu.hsecfg().modify(|_, w| w.en().set_bit());
    for _ in 0..100_000 {
        unsafe { core::arch::asm!("nop") };
    }

    p.rcu.cgcfgahb().modify(|_, w| w.gpioben().set_bit());
    p.rcu.rstdisahb().modify(|_, w| w.gpioben().set_bit());
    p.rcu.cgcfgapb2().modify(|_, w| w.uart0en().set_bit());
    p.rcu.rstdisapb2().modify(|_, w| w.uart0en().set_bit());

    // B0/B1 -> UART0 (alternate function 3).
    p.gpiob.altfuncnum0().modify(|_, w| w.pin0().af3().pin1().af3());
    p.gpiob.altfuncset().write(|w| w.pin0().set_bit().pin1().set_bit());

    p.rcu.uartcfg(0).value().modify(|_, w| {
        w.clksel().srcclk();   // HSE
        w.clken().set_bit();
        w.rstdis().set_bit()
    });

    // 9600 baud from 27 MHz: 27_000_000 / (16 * 9600) = 175.78125
    // -> IBRD = 175, FBRD = 0.78125 * 64 = 50 (exactly 9600 baud).
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

// Decimal output of a u32 (there is no printf).
fn uart0_send_u32(p: &pac::Peripherals, mut n: u32) {
    if n == 0 {
        uart0_send_byte(p, b'0');
        return;
    }
    let mut buf = [0u8; 10]; // u32 has at most 10 decimal digits
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

// TMR0 as a free-running counter clocked like the core (HSI after reset), no interrupts.
fn tmr0_counter_start(p: &pac::Peripherals) {
    p.rcu.cgcfgapb1().modify(|_, w| w.tmr0en().set_bit());
    p.rcu.rstdisapb1().modify(|_, w| w.tmr0en().set_bit());
    p.tmr0.period().write(|w| unsafe { w.val().bits(0xFFFF_FFFF) });
    p.tmr0.ctrl().modify(|_, w| w.mode().up());
    p.siu.tmren().modify(|_, w| w.tmr0en().set_bit());
}

#[entry]
fn main() -> ! {
    let p = unsafe { pac::Peripherals::steal() };

    uart0_init(&p);
    tmr0_counter_start(&p);

    uart0_send_str(&p, "\r\nHello from K1921VG1T!\r\n");
    uart0_send_str(&p, "Measuring HSI (wait ~2s)...\r\n");

    // HSI self-measurement: the core and TMR0 run from HSI, the UART runs from the accurate
    // HSE crystal. Sending N bytes at 9600 baud takes exactly N * 10 / 9600 s (10 bits per
    // byte), so the number of TMR0 ticks during the transfer gives the HSI frequency.
    // Wait until the text above has left the 32-byte TX FIFO, otherwise its bytes are
    // counted as part of the N bytes and the result comes out too high.
    const N: u32 = 2000;
    while p.uart0.fr().read().busy().bit_is_set() {}
    let t0 = p.tmr0.count().read().val().bits();
    for _ in 0..N {
        uart0_send_byte(&p, b'.');
    }
    while p.uart0.fr().read().busy().bit_is_set() {} // wait until the last byte is sent
    let t1 = p.tmr0.count().read().val().bits();

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
