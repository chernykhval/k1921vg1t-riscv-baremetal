#![no_std]
#![no_main]

use riscv_rt::entry;
use core::ptr::write_volatile;
use core::panic::PanicInfo;

// --- Адреса регистров (из SVD: base + offset) -------------------------------
const RCU_CGCFGAHB:   *mut u32 = 0x5000_4000 as *mut u32; // такт периферии AHB
const RCU_RSTDISAHB:  *mut u32 = 0x5000_4010 as *mut u32; // снятие сброса AHB
const GPIOC_OUTENSET: *mut u32 = 0x4000_202C as *mut u32; // линии в выход
const GPIOC_DATAOUT:  *mut u32 = 0x4000_2004 as *mut u32; // значение на выход

const GPIOCEN: u32 = 1 << 2;    // бит порта C в регистрах RCU
const LEDS:    u32 = 0xF000;    // C12..C15
const LED1:    u32 = 1 << 12;   // C12 (active-high: 1 зажигает)

#[entry]
fn main() -> ! {
    unsafe {
        // Прямая запись в регистры. unsafe — потому что Rust не может проверить,
        // что за этими "сырыми" адресами реально есть железо (см. пояснение).
        write_volatile(RCU_CGCFGAHB,  GPIOCEN);   // такт на порт C
        write_volatile(RCU_RSTDISAHB, GPIOCEN);   // снять сброс с порта C
        write_volatile(GPIOC_OUTENSET, LEDS);     // C12..C15 -> выход
        // write_volatile(GPIOC_DATAOUT,  LED1);     // зажечь LED1

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
