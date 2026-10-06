#![no_std]
#![no_main]

use core::panic::PanicInfo;
use k1921vg1t_pac as pac;
use riscv_rt::entry;
use core::ptr::{read_volatile, write_volatile};
use riscv::interrupt::Interrupt;

const PLIC_PRIORITY_TMR0: *mut u32 = 0xFE00_0040 as *mut u32;
const PLIC_MODE_TMR0:     *mut u32 = 0xFE1F_0040 as *mut u32;
const PLIC_ENABLE_M0:     *mut u32 = 0xFE00_2000 as *mut u32;
const PLIC_THRESHOLD_M0:  *mut u32 = 0xFE20_0000 as *mut u32;
const PLIC_CLAIM_M0: *mut u32 = 0xFE20_0004 as *mut u32;

fn tmr0_init(p: &pac::Peripherals, period: u32) {
    p.rcu.cgcfgapb1().modify(|_, w| w.tmr0en().set_bit());
    p.rcu.rstdisapb1().modify(|_, w| w.tmr0en().set_bit());
    p.tmr0.period().write(|w| unsafe { w.val().bits(period - 1)});
    p.tmr0.ctrl().modify(|_, w| w.mode().up());
    p.tmr0.im().write(|w| w.tmr().set_bit());
    p.siu.tmren().modify(|_, w| w.tmr0en().set_bit());
}

fn plic_init() {
    unsafe {
        write_volatile(PLIC_PRIORITY_TMR0,  0x1);
        write_volatile(PLIC_MODE_TMR0,  0x1);
        write_volatile(PLIC_ENABLE_M0, 0x1 << 16);
        write_volatile(PLIC_THRESHOLD_M0, 0x0);
    }
}

#[entry]
fn main() -> ! {
    let p = unsafe { pac::Peripherals::steal() };

      // включить порт C и линии LED на выход (как в мигалке — скопируй оттуда)
    p.rcu.cgcfgahb().modify(|_, w| w.gpiocen().set_bit());
    p.rcu.rstdisahb().modify(|_, w| w.gpiocen().set_bit());
    p.gpioc.outenset().write(|w| {
        w.pin12().set_bit();
        w.pin13().set_bit();
        w.pin14().set_bit();
        w.pin15().set_bit()
    });

    tmr0_init(&p, 1_000_000);   // период подберём потом
    plic_init();

    unsafe {
        riscv::register::mie::enable(Interrupt::MachineExternal);  // MEIE — конкретный бит
        riscv::interrupt::enable();                                // mstatus.MIE — глобально
    }

    loop {}   // пока пусто — прерывания добавим следующими шагами
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[riscv_rt::core_interrupt(Interrupt::MachineExternal)]
fn machine_external() {
    unsafe {
          // 1. CLAIM: спросить PLIC, кто звонил (вернёт 16 = TMR0)
        let src = read_volatile(PLIC_CLAIM_M0);

          // 2. РАБОТА: мигнуть LED1 (DATAOUTTGL переключает бит)
        let p = pac::Peripherals::steal();
        p.gpioc.dataouttgl().write(|w| w.val().bits(1 << 12));

          // 3. СБРОСИТЬ ФЛАГ ТАЙМЕРА (иначе прерывание зациклится!)
        p.tmr0.ic().write(|w| w.tmr().set_bit());

          // 4. COMPLETE: доложить PLIC, что обработал (иначе следующее не пройдёт)
        write_volatile(PLIC_CLAIM_M0, src);
    }
}
