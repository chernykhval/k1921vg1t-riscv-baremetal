# k1921vg1t-pac

[English version](README.en.md)

Rust-крейт доступа к периферии (PAC, Peripheral Access Crate) микроконтроллера К1921ВГ1Т
(АО «НИИЭТ», RISC-V). Сгенерирован инструментом svd2rust из SVD-описания SDK производителя.
SVD перед генерацией пришлось исправить. Найденные дефекты и все правки описаны в
[SVD_FIXES.md](SVD_FIXES.md).

Крейт предназначен для `no_std`-программ под цель `riscv32imac-unknown-none-elf`. На crates.io не
опубликован, подключается по пути.

## Содержимое

| Файл | Назначение |
|---|---|
| `K1921VG1T.patched.svd` | исправленное SVD-описание — исходный файл для генерации |
| `K1921VG1T.svd.diff` | все изменения относительно SVD из SDK (`9fa40f85be5b`) |
| `SVD_FIXES.md`, `SVD_FIXES.en.md` | найденные дефекты SVD, основания правок, способ проверки (рус. и англ.) |
| `src/`, `build.rs`, `device.x` | вывод svd2rust 0.37.1 (разбит form, отформатирован rustfmt), без ручных правок |
| `Cargo.toml` | описание крейта (написано вручную) |

## Подключение

```toml
[dependencies]
k1921vg1t-pac = { path = "../k1921vg1t-pac" }
```

Сокращённый фрагмент `rust_blink_pac` — включить тактирование и снять сброс порта GPIOC, вывести 1
на линию C12:

```rust
use k1921vg1t_pac as pac;

let p = unsafe { pac::Peripherals::steal() };
p.rcu.cgcfgahb().modify(|_, w| w.gpiocen().set_bit());
p.rcu.rstdisahb().modify(|_, w| w.gpiocen().set_bit());
p.gpioc.outenset().write(|w| w.pin12().set_bit());
p.gpioc.dataout().write(|w| unsafe { w.val().bits(0x1000) });
```

Регистры-массивы и кластеры доступны по индексу: `p.rcu.uartcfg(0).value()`, `p.adc.actl(3).ctrl()`.
Имена `value` и `ctrl` введены при исправлении SVD (п. 3 и 8 в SVD_FIXES.md).

## Особенности

- `write()` начинает запись со значения сброса регистра, `write_with_zero()` (unsafe) — с нуля.
  Для регистров с ненулевым сбросом это важно. Пример — `RCU.PLL[0].CFG` в `rust_pll`: значение сброса содержит
  бит отключения PLL.
- Контроллер прерываний PLIC в SVD не описан, поэтому в PAC его нет. Пример `rust_timer_irq`
  работает с ним по адресам напрямую.
- Cargo-feature `rt` подключает массив `__EXTERNAL_INTERRUPTS` и `device.x` для riscv-rt 0.13. С ней
  крейт собирается, но примеры репозитория её не используют, и на плате она не проверялась. Примеры
  подключают riscv-rt 0.18 напрямую.
- Код сгенерирован без файла настроек svd2rust для RISC-V, в «устаревшем» режиме описания прерываний.
  svd2rust выдаёт об этом два предупреждения: `No settings file provided for RISC-V target. Using legacy
  interrupts rendering` и `Please, consider migrating your PAC to riscv 0.12.0 or later`.

## Перегенерация

Команды, которыми получен `src/`, приведены в [SVD_FIXES.md](SVD_FIXES.md#исходные-данные).
При тех же версиях инструментов результат совпадает с репозиторием побайтно (проверяется в CI на Linux и Windows).

## Лицензия

Крейт объединяет материалы с разными лицензиями: SVD-описание НИИЭТ (открытая лицензия
правообладателя) с изменениями автора (CC BY 4.0), код, сгенерированный из него (лицензия НИИЭТ и
CC BY 4.0), и шаблонный код svd2rust (MIT OR Apache-2.0). Подробно — в [NOTICE.md](../../NOTICE.md).
