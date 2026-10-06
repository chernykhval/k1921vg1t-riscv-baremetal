#![no_std]
#![no_main]

use core::panic::PanicInfo;
use k1921vg1t_pac as pac;
use riscv_rt::entry;

// UART0 output, 9600 8N1. The UART is clocked from the 27 MHz HSE crystal, so switching
// the core to the PLL does not change the baud rate.
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

// System clock self-measurement: TMR0 counts system clock ticks while N bytes are sent
// over the HSE-clocked UART; the transfer takes exactly N * 10 / 9600 s.
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

// Switch the system clock to PLL0 using the NIIET SDK settings for a 27 MHz HSE
// (system_k1921vg1t.c). The SDK comment says "FOUT = 204 000 000 Hz", but by the SDK's
// own formula F = HSE * NDIV / ((RDIV + 1) * (DIV1A + 1) * (DIV1B + 1)) (PREDIV is not
// used with VCOMODE = 1) they give 27 MHz * 100 / (3 * 2 * 3) = 150 MHz;
// about 150.9 MHz was measured.
// Wrong settings hang the chip; the order of the steps matters.
fn pll_init(p: &pac::Peripherals) {
    // 1. The PLL reference (HSE) must be running.
    p.rcu.hsecfg().modify(|_, w| w.en().set_bit());
    for _ in 0..100_000 {
        unsafe { core::arch::asm!("nop") };
    }

    // 2. Dividers and multiplier. write_with_zero(): the reset values are not used.
    unsafe {
        p.rcu.pll(0).div().write_with_zero(|w| {
            w.div1a().bits(1);
            w.div1b().bits(2);
            w.prediv().bits(1);
            w.nnclr().set_bit(); // enable the N divider
            w.rnclr().set_bit(); // enable the R divider
            w.rdiv().bits(2);
            w.ndiv().bits(100)
        });
    }
    p.rcu.pll(0).mod_().write(|w| unsafe { w.mod_().bits(1) });
    p.rcu.pll(0).frac().write(|w| unsafe { w.frac().bits(1) });
    // The reset value of CFG has PD (power-down) = 1; write_with_zero() clears it,
    // a plain write() would keep it and the PLL would never start.
    unsafe {
        p.rcu.pll(0).cfg().write_with_zero(|w| {
            w.fouten().set_bit(); // output enable
            w.pfd().bits(3);
            w.clksel().clear_bit(); // 0 = HSE
            w.vcomode().bits(1);
            w.cp().bits(9);
            w.st().bits(0) // integer divider
        });
    }

    // 3. Give the PLL time to lock.
    for _ in 0..100_000 {
        unsafe { core::arch::asm!("nop") };
    }

    // 4. Increase the flash latency before raising the clock (the SDK sets LAT = 2 too).
    p.flashm.ctrl().modify(|_, w| unsafe { w.lat().bits(2) });

    // 5. Switch the system clock to PLL0 (SRC = 2) and wait for the switch.
    p.rcu.sysclkcfg().write(|w| unsafe { w.src().bits(2) });
    while p.rcu.clkstat().read().src().bits() != 2 {}
}

#[entry]
fn main() -> ! {
    let p = unsafe { pac::Peripherals::steal() };
    uart0_init(&p);
    tmr0_counter_start(&p);

    // Before: the core runs from HSI after reset.
    uart0_send_str(&p, "\r\nBefore PLL: ");
    let before = measure_sysclk(&p);
    uart0_send_u32(&p, before);
    uart0_send_str(&p, " Hz\r\n");

    pll_init(&p);

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
