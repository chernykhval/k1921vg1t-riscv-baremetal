// main.rs — первая bare-metal программа на Rust: зажечь LED1 и застыть.
// Аналог самой первой C-программы, только тут мы ещё и сами пишем startup.

#![no_std]   // без стандартной библиотеки (нет ОС) — только core
#![no_main]  // не использовать обычную точку входа Rust (main с рантаймом)

use core::arch::global_asm;
use core::ptr::write_volatile;
use core::panic::PanicInfo;

// --- Стартовый код (аналог startup_k1921vg1t.S) -----------------------------
// Ассемблер прямо в Rust. Задаёт указатель стека и прыгает в rust_main.
// Секция .init -> линкер кладёт это на адрес 0x0 -> сюда приходит процессор
// после сброса.
global_asm!(
    ".section .init",
    ".globl _start",
    "_start:",
    "  la   sp, _stack_top",   // sp = вершина стека (символ из link.x)
    "  call rust_main",        // прыжок в наш код на Rust
    "1:",
    "  j 1b",                   // если rust_main вернётся — вечный цикл
);

// --- Адреса регистров (из SVD: base + offset) -------------------------------
const RCU_CGCFGAHB:   *mut u32 = 0x5000_4000 as *mut u32; // такт периферии AHB
const RCU_RSTDISAHB:  *mut u32 = 0x5000_4010 as *mut u32; // снятие сброса AHB
const GPIOC_OUTENSET: *mut u32 = 0x4000_202C as *mut u32; // линии в выход
const GPIOC_DATAOUT:  *mut u32 = 0x4000_2004 as *mut u32; // значение на выход

const GPIOCEN: u32 = 1 << 2;    // бит порта C в регистрах RCU
const LEDS:    u32 = 0xF000;    // C12..C15
const LED1:    u32 = 1 << 12;   // C12 (active-high: 1 зажигает)

// --- Наша программа ---------------------------------------------------------
// extern "C" + no_mangle: чтобы ассемблер выше нашёл функцию по имени rust_main.
// Тип "-> !" означает "никогда не возвращается" (вечный цикл в конце).
#[no_mangle]
pub extern "C" fn rust_main() -> ! {
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
    // loop {}   // застыть
}

// --- Обработчик паники (обязателен в no_std) --------------------------------
// Если программа "паникует" (аналог необработанной ошибки) — просто зависаем.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
