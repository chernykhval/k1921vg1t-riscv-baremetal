#![no_std]
#![no_main]

use riscv_rt::entry;
use core::ptr::write_volatile;
use core::panic::PanicInfo;

// Register addresses (base + offset from the SVD).
const RCU_CGCFGAHB:   *mut u32 = 0x5000_4000 as *mut u32; // AHB peripheral clock enable
const RCU_RSTDISAHB:  *mut u32 = 0x5000_4010 as *mut u32; // AHB peripheral reset release
const GPIOC_OUTENSET: *mut u32 = 0x4000_202C as *mut u32; // set pins as outputs
const GPIOC_DATAOUT:  *mut u32 = 0x4000_2004 as *mut u32; // output data

const GPIOCEN: u32 = 1 << 2;    // GPIOC bit in the RCU registers
const LEDS:    u32 = 0xF000;    // C12..C15
const LED1:    u32 = 1 << 12;   // C12; the LEDs are active-high (verified on this
                                // board; the SDK BSP assumes active-low)

#[entry]
fn main() -> ! {
    unsafe {
        write_volatile(RCU_CGCFGAHB,  GPIOCEN);
        write_volatile(RCU_RSTDISAHB, GPIOCEN);
        write_volatile(GPIOC_OUTENSET, LEDS);

        loop {
            write_volatile(GPIOC_DATAOUT, LED1);
            for _ in 0..500_000 {
                core::arch::asm!("nop");
            }
            write_volatile(GPIOC_DATAOUT, 0);
            for _ in 0..500_000 {
                core::arch::asm!("nop");
            }
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
