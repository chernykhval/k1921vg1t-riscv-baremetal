// LED1 blink through the PAC: the same steps as rust_blink_rt, but with named registers.
#![no_std]
#![no_main]

use core::panic::PanicInfo;
use k1921vg1t_pac as pac;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    // steal(): take the peripherals without the singleton check (single-threaded program).
    let p = unsafe { pac::Peripherals::steal() };

    // Enable the GPIOC clock and release its reset (read-modify-write of one bit).
    p.rcu.cgcfgahb().modify(|_, w| w.gpiocen().set_bit());
    p.rcu.rstdisahb().modify(|_, w| w.gpiocen().set_bit());

    // C12..C15 as outputs.
    p.gpioc.outenset().write(|w| {
        w.pin12().set_bit();
        w.pin13().set_bit();
        w.pin14().set_bit();
        w.pin15().set_bit()
    });

    loop {
        // DATAOUT has a 16-bit `val` field; bit 12 is LED1.
        p.gpioc.dataout().write(|w| unsafe { w.val().bits(0x1000) });
        for _ in 0..500_000 {
            unsafe { core::arch::asm!("nop") };
        }
        p.gpioc.dataout().write(|w| unsafe { w.val().bits(0x0000) });
        for _ in 0..500_000 {
            unsafe { core::arch::asm!("nop") };
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
