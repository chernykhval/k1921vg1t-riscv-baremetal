// Мигалка на PAC: те же действия, но через ИМЕНОВАННЫЕ регистры,
// а не сырые адреса. Сравни с rust_blink_rt/src/main.rs.
#![no_std]
#![no_main]

use core::panic::PanicInfo;
use k1921vg1t_pac as pac; // наш сгенерированный крейт доступа к периферии
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    // Забираем все "периферии" чипа одним объектом.
    // steal() = "дай доступ, я отвечаю за уникальность" (для простой мигалки ок).
    let p = unsafe { pac::Peripherals::steal() };

    // Тактирование и снятие сброса порта C — вместо write_volatile по адресам RCU.
    // modify = прочитать-изменить-записать; трогаем ТОЛЬКО бит gpiocen, остальное как было.
    p.rcu.cgcfgahb().modify(|_, w| w.gpiocen().set_bit());
    p.rcu.rstdisahb().modify(|_, w| w.gpiocen().set_bit());

    // C12..C15 -> выход. У OUTENSET поля по пинам; ставим четыре бита.
    p.gpioc.outenset().write(|w| {
        w.pin12().set_bit();
        w.pin13().set_bit();
        w.pin14().set_bit();
        w.pin15().set_bit()
    });

    loop {
        // Зажечь LED1 (бит 12). У DATAOUT поле val (16 бит) — пишем значение.
        p.gpioc.dataout().write(|w| unsafe { w.val().bits(0x1000) });
        for _ in 0..500_000 {
            unsafe { core::arch::asm!("nop") };
        }
        // Погасить.
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
