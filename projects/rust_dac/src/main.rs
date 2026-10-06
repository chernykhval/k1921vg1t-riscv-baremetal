#![no_std]
#![no_main]

use core::panic::PanicInfo;
use k1921vg1t_pac as pac;
use riscv_rt::entry;

// One sine period, 64 points: round(2048 + 2047 * sin(2 * pi * i / 64)), 12-bit DAC codes.
static SINE: [u16; 64] = [
    2048,2249,2447,2642,2831,3013,3185,3347,
    3495,3630,3750,3853,3939,4007,4056,4085,
    4095,4085,4056,4007,3939,3853,3750,3630,
    3495,3347,3185,3013,2831,2642,2447,2249,
    2048,1847,1649,1454,1265,1083, 911, 749,
    601, 466, 346, 243, 157,  89,  40,  11,
    1,  11,  40,  89, 157, 243, 346, 466,
    601, 749, 911,1083,1265,1454,1649,1847,
];

fn dac0_init(p: &pac::Peripherals) {
    // Clock enable and reset release.
    p.rcu.cgcfgapb1().modify(|_, w| w.dac0en().set_bit());
    p.rcu.rstdisapb1().modify(|_, w| w.dac0en().set_bit());

    // Internal reference, 2.5 V setting.
    p.dac0.bgctrl().modify(|_, w| w.en().set_bit().adjust().v_2p5());

    // Enable the DAC with the internal reference and the output buffer;
    // no hardware wave generator, conversions are started by software.
    p.dac0.control().modify(|_, w| {
        w.disable().clear_bit();
        w.ext_ref().clear_bit();
        w.buf_en().set_bit();
        w.wave().none();
        w.tsel().swtrig()
    });

    // Start from code 0 (0 V); the sine is written in the main loop.
    p.dac0.data12().write(|w| unsafe { w.val().bits(0)} );

    // Software trigger: start the conversion.
    p.dac0.swtrig().write(|w| w.swtrig().set_bit());
}

#[entry]
fn main() -> ! {
    let p = unsafe { pac::Peripherals::steal() };

    dac0_init(&p);

    loop {
        for &v in SINE.iter() {
            p.dac0.data12().write(|w| unsafe { w.val().bits(v) });
            p.dac0.swtrig().write(|w| w.swtrig().set_bit());
            for _ in 0..250_000 { unsafe { core::arch::asm!("nop") } } // slow enough for a multimeter
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
