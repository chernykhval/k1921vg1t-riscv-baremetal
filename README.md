# К1921ВГ1Т: bare-metal на C и Rust

[English version](README.en.md)

[![build](https://github.com/chernykhval/k1921vg1t-riscv-baremetal/actions/workflows/build.yml/badge.svg)](https://github.com/chernykhval/k1921vg1t-riscv-baremetal/actions/workflows/build.yml)

Примеры bare-metal программ для микроконтроллера К1921ВГ1Т (АО «НИИЭТ», ядро RISC-V SCR5,
RV32IMAFDCP) на C и на Rust (`no_std`), Rust-крейт доступа к периферии (PAC), сгенерированный из
исправленного SVD-описания производителя, и разбор найденных в этом описании дефектов.
Работа выполнена в рамках программы DEVBOARDS Российского Альянса RISC-V.

## Содержимое

| Путь | Что это |
|---|---|
| [`projects/k1921vg1t-pac/`](projects/k1921vg1t-pac/) | PAC для К1921ВГ1Т, исправленный SVD и [разбор дефектов SVD](projects/k1921vg1t-pac/SVD_FIXES.md) |
| `projects/rust_*/` | 9 примеров на Rust, от мигания светодиодом до ГОСТ-шифрования и PLL |
| `projects/run_leds/`, `projects/binary_counter/`, `projects/led_breath/` | примеры на C, собираются с SDK производителя |
| [`openocd/`](openocd/) | конфигурации OpenOCD для адаптера FT2232HL Board и К1921ВГ1Т |
| [`Taskfile.yml`](Taskfile.yml) | команды сборки и прошивки Rust-примеров ([Task](https://taskfile.dev)) |
| [`NOTICE.md`](NOTICE.md), [`LICENSE`](LICENSE), `LICENSES/` | лицензии |

## Примеры на Rust

| Шаг | Проект | Что делает |
|---|---|---|
| 1 | `rust_blink` | мигает LED1 без единого крейта: свой код запуска и свой скрипт компоновщика |
| 2 | `rust_blink_rt` | то же мигание на среде выполнения `riscv-rt` |
| 3 | `rust_blink_pac` | то же мигание через PAC: регистры по именам вместо адресов |
| 4 | `rust_timer_irq` | переключает LED1 из прерывания TMR0 (TMR0 → PLIC → внешнее прерывание machine-режима) |
| 5 | `rust_uart` | вывод текста по UART0 (9600 8N1) и измерение частоты внутреннего генератора HSI |
| 6 | `rust_adc` | термометр: АЦП измеряет встроенный датчик температуры, вывод по UART |
| 7 | `rust_dac` | генератор сигнала на ЦАП0 (синус из 64 точек), выход на разъём SMA «DAC 0» |
| 8 | `rust_crypto` | аппаратное шифрование «Кузнечик» блоком CRYPTO, сверка с тестовым вектором ГОСТ Р 34.12-2015 |
| 9 | `rust_pll` | переключение системной частоты на PLL0 (кварц 27 МГц → 150 МГц) и её измерение |

Шаги 3–9 используют [PAC](projects/k1921vg1t-pac/). Примеры собираются под цель
`riscv32imac-unknown-none-elf`.

## Примеры на C

| Проект | Что делает |
|---|---|
| `run_leds` | изменённая копия примера `run_leds` из SDK НИИЭТ (изменения перечислены в начале `app/main.c`); сообщения `printf` выводятся в UART0 на скорости 115200 бод (настройка retarget в SDK) |
| `binary_counter` | считает нажатия кнопки USER BTN (спад уровня на выводе B3, без подавления дребезга) и показывает число от 0 до 15 в двоичном виде на LED1–LED4; работа с регистрами GPIO напрямую |
| `led_breath` | плавно увеличивает и уменьшает яркость LED1 программным ШИМ (время включения пропорционально квадрату шага); LED2 горит, пока яркость растёт |

У `binary_counter` есть второй вариант сборки, `Makefile_fit`: в нём исходные файлы SDK перечислены
явно, а не подставляются по шаблону.

## Оборудование

- Отладочная плата НИИЭТ с К1921ВГ1Т (КФДЛ.441461.050 rev 1.1; в SDK — `NIIET-DEV-K1921VG1T`).
  Питание 12 В через разъём XS5. Перемычка XP5 «LEDS ON» должна стоять, иначе
  светодиоды LED1–LED4 не подключены к выводам C12–C15.
- JTAG-адаптер [FT2232HL Board V3.6](https://github.com/arm8686/FT2232HL-Board). JTAG на нём разведён
  на канал B (Interface 1). Для Interface 1 в Windows нужно установить драйвер WinUSB программой
  [Zadig](https://zadig.akeo.ie/); Interface 0 не трогать.
- Для вывода по UART — кабель USB-C в разъём XS4 «USB-UART» платы (мост CH340B, UART0 чипа).

Подключение адаптера к разъёму XP7 «JTAG» платы (прямое, без перекрещивания TDI/TDO):

| Сигнал | FT2232HL Board, CN4 | Плата, XP7 |
|---|---|---|
| TMS | 7 | 2 |
| TCK | 9 | 4 |
| TDO | 13 | 6 |
| TDI | 5 | 8 |
| nRST | 15 | 10 |
| GND | GND | 3, 5 или 9 |

Вывод 1 XP7 (+3,3 В) к адаптеру не подключается.

## Программы

| Что | Где взять | Проверено с версией |
|---|---|---|
| Rust | [rustup](https://rustup.rs/); версия и цель закреплены в `rust-toolchain.toml` | 1.97.1 |
| OpenOCD НИИЭТ | [gitflic.ru/project/niiet/openocd](https://gitflic.ru/project/niiet/openocd) (исходники и релизы) | `-00247-g90632ca40-dirty (2025-08-01-14:40)` |
| Task (необязательно) | [taskfile.dev](https://taskfile.dev/) | 3.54.0 (нужна 3.38 или новее) |
| Для C: SDK НИИЭТ | [gitflic.ru/project/niiet/k1921vg1t_sdk](https://gitflic.ru/project/niiet/k1921vg1t_sdk) | коммит `9fa40f85be5b` |
| Для C: компилятор | [xPack riscv-none-elf-gcc](https://github.com/xpack-dev-tools/riscv-none-elf-gcc-xpack/releases) | 13.4.0-1 |
| Для C: make | GNU make (правило `clean` рассчитано на командную строку Windows) | GNU Make 3.81 |

Нужен именно OpenOCD НИИЭТ: конфигурация использует драйвер flash-памяти `k1921vg1t` из этого форка.
Документация на микроконтроллер (руководство пользователя, errata, на русском языке) — в репозитории
[gitflic.ru/project/niiet/niiet](https://gitflic.ru/project/niiet/niiet).

## Быстрый старт

Команды выполняются из корня репозитория.

1. Установить [rustup](https://rustup.rs/). Нужная версия Rust (1.97.1), цель
   `riscv32imac-unknown-none-elf` и rustfmt указаны в файле [`rust-toolchain.toml`](rust-toolchain.toml);
   rustup устанавливает их сам при первом запуске `cargo` в репозитории.

2. Подключить адаптер и плату, включить питание. Проверить связь с чипом:

   ```sh
   task scan
   ```

   или без Task:

   ```sh
   openocd -s openocd -f ft2232hl_board.cfg -f k1921vg1t_singlecore_dev.cfg -c init -c shutdown
   ```

   Признак успеха в выводе: `0xdeb15001`, `Examined RISC-V core; found 1 harts`,
   `hart 0: XLEN=32, misa=0x4014912d`.

3. Собрать и прошить пример:

   ```sh
   task run P=rust_blink_pac
   ```

   Если `openocd` нет в `PATH`, путь к нему передаётся так:
   `task run P=rust_blink_pac OPENOCD=<путь>/openocd.exe`.

   Без Task то же делается командами:

   ```sh
   cd projects/rust_blink_pac && cargo build --release && cd ../..
   openocd -s openocd -f connect_ft2232_jtag.cfg -c halt \
     -c "flash write_image erase projects/rust_blink_pac/target/riscv32imac-unknown-none-elf/release/rust_blink_pac" \
     -c "verify_image projects/rust_blink_pac/target/riscv32imac-unknown-none-elf/release/rust_blink_pac" \
     -c shutdown
   ```

   Затем нажать кнопку RESET (SB2) на плате. После сброса мигает LED1.

4. Для примеров с выводом по UART (шаги 5, 6, 8, 9) открыть терминал на COM-порт моста CH340B:
   9600 бод, 8 бит, без чётности, 1 стоповый бит.

## Сборка примеров на C

SDK НИИЭТ клонируется рядом с этим репозиторием, а не внутрь него (так его по умолчанию ищут
Makefile). Из корня репозитория:

```sh
git clone https://gitflic.ru/project/niiet/k1921vg1t_sdk.git ../k1921vg1t_sdk
git -C ../k1921vg1t_sdk checkout 9fa40f85be5b
```

Сборка (без `COMPILER_PATH` компилятор берётся из `PATH`):

```sh
make -C projects/run_leds COMPILER_PATH=<путь к xpack>/bin/
```

Если SDK лежит в другом месте, путь к нему передаётся параметром `SDK_DIR` (абсолютный путь или
путь относительно каталога проекта):

```sh
make -C projects/run_leds SDK_DIR=<путь к SDK>
```

Собранная прошивка — `projects/run_leds/build/run_leds.elf` (у `Makefile_fit` — каталог `build_fit/`). Она прошивается так же, как в шаге 3:

```sh
openocd -s openocd -f connect_ft2232_jtag.cfg -c halt \
  -c "flash write_image erase projects/run_leds/build/run_leds.elf" \
  -c "verify_image projects/run_leds/build/run_leds.elf" -c shutdown
```

Затем нажать кнопку RESET (SB2) на плате.

## Особенности

- Частота TCK для К1921ВГ1Т — не более 200–250 кГц (документ НИИЭТ «Быстрый старт с микроконтроллерами
  К1921ВГ015, К1921ВГ1Т…», с. 5); в конфигурациях репозитория задано 200 кГц.
- В конфигурации OpenOCD нет управления сбросом (`reset_config none`), поэтому после прошивки
  плату нужно сбросить кнопкой SB2.
- Если OpenOCD сообщает `libusb_open() failed with LIBUSB_ERROR_NOT_FOUND`, на канале B адаптера
  вместо WinUSB снова стоит драйвер FTDI: его нужно заново заменить программой Zadig (см.
  «Оборудование»). При этой ошибке OpenOCD завершается с кодом 0, поэтому об успехе можно судить
  только по выводу.
- Примеры на C проверены с SDK `k1921vg1t_sdk` на коммите `9fa40f85be5b`. С более новыми версиями
  SDK и с новым SDK производителя [riscv_cmake_sdk](https://gitflic.ru/project/niiet/riscv_cmake_sdk)
  совместимость не проверялась.
- В версии SDK `9fa40f85be5b` блок настройки PLL для кварца 27 МГц подписан «FOUT = 204 000 000 Hz»,
  а заданные в нём делители дают 150 МГц; в более поздних версиях SDK подпись исправлена на
  150 МГц. Пример `rust_pll` настраивает PLL0 на 150 МГц.
- В `k1921vg1t_sdk` на коммите `9fa40f85be5b` в конфигурации OpenOCD указаны JTAG ID `0xDEB05001` и
  частота 2000 кГц; конфигурация этого репозитория использует ID `0xDEB15001`, прочитанный с чипа,
  и 200 кГц. В более поздних версиях SDK ID исправлен.

## Лицензия

© 2026 Черных В.А. Материалы репозитория распространяются на условиях лицензии
[CC BY 4.0](https://creativecommons.org/licenses/by/4.0/legalcode.ru), если не указано иное.
Сторонние файлы (SVD-описание и пример НИИЭТ, шаблонный код svd2rust, конфигурации OpenOCD)
распространяются на условиях своих лицензий — подробно в [NOTICE.md](NOTICE.md).
