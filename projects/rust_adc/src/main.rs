#![no_std]
#![no_main]

use core::panic::PanicInfo;
use k1921vg1t_pac as pac;
use riscv_rt::entry;

// UART0 output, 9600 8N1 from the 27 MHz HSE crystal (same as rust_uart).
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

// Decimal output of a u32 (there is no printf).
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

// The on-chip temperature sensor (TSENSOR) is connected to channel 63 of analog block ADC3.
const TSENSOR_CH: u8 = 63;

fn adc_init(p: &pac::Peripherals) {
    // ADC clock: CLKSEL = 1 (SRCCLK = HSE), no divider.
    p.rcu.adccfg().modify(|_, w| unsafe {
        w.clksel().bits(1);
        w.diven().clear_bit();
        w.clken().set_bit();
        w.rstdis().set_bit()
    });
    p.rcu.cgcfgahb().modify(|_, w| w.adcen().set_bit());
    p.rcu.rstdisahb().modify(|_, w| w.adcen().set_bit());

    // Analog front end of ADC3: internal LDO, 12-bit resolution, no calibration.
    p.adc.actl(3).ctrl().modify(|_, w| {
        w.adcenldo().set_bit();
        w.rangeldo().set_bit();
        w.dislvl().clear_bit();
        w.selres()._12bit();
        w.calen().clear_bit();
        w.adcen().set_bit()
    });
    for _ in 0..500 {
        unsafe { core::arch::asm!("nop") }; // let the analog front end settle
    }
    p.adc.actl(3).ctrl().modify(|_, w| w.adcstart().set_bit());

    // Enable the temperature sensor. SCALE = 1 is assumed by the conversion formula below.
    p.siu.tsensctrl().modify(|_, w| {
        w.scale().set_bit();
        w.dis().clear_bit()
    });

    // Sequencer 0: a single software-triggered conversion of channel 63.
    p.adc.emux().modify(|_, w| unsafe { w.em0().bits(0) });
    p.adc.seq(0).srqsel0().modify(|_, w| unsafe { w.rq0().bits(TSENSOR_CH) });
    p.adc.seq(0).srqctl().modify(|_, w| unsafe {
        w.rqmax().bits(0); // queue of one request
        w.qavgen().clear_bit()
    });
    // Extra channel delay: the temperature sensor needs settling time.
    p.adc.chdelay(TSENSOR_CH as usize).value().write(|w| unsafe { w.delay().bits(2700) });

    p.adc.seqsync().modify(|_, w| w.sync0().set_bit());
    p.adc.seqen().modify(|_, w| w.seqen0().set_bit());

    while p.adc.actl(3).ctrl().read().adcrdy().bit_is_clear() {}
}

// One TSENSOR conversion; returns the raw 12-bit ADC value.
fn adc_read_tsensor(p: &pac::Peripherals) -> u16 {
    // GSYNC starts all sequencers that have SYNC set.
    p.adc.seqsync().modify(|_, w| w.gsync().set_bit());
    // Poll the FIFO fill level (SFLOAD). In testing, polling the SEQRIS0 flag instead
    // hung on the second conversion.
    while p.adc.seq(0).sfload().read().val().bits() == 0 {}
    p.adc.seq(0).sfifo().read().data().bits() // reading pops the value from the FIFO
}

// Temperature in tenths of a degree Celsius (valid for TSENSOR SCALE = 1).
fn raw_to_temp_x10(raw: u16) -> i32 {
    let v_mv = raw as i32 * 3300 / 4096;
    574713 * (793 - v_mv) / 100000
}

#[entry]
fn main() -> ! {
    let p = unsafe { pac::Peripherals::steal() };
    uart0_init(&p);
    adc_init(&p);

    loop {
        let raw = adc_read_tsensor(&p);
        let t10 = raw_to_temp_x10(raw);

        uart0_send_str(&p, "T = ");
        uart0_send_u32(&p, (t10 / 10) as u32);
        uart0_send_str(&p, ".");
        uart0_send_u32(&p, (t10 % 10) as u32);
        uart0_send_str(&p, " C\r\n");

        for _ in 0..3_000_000 { unsafe { core::arch::asm!("nop") } }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
