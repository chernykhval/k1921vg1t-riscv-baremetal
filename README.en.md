# K1921VG1T: bare-metal in C and Rust

[Русская версия](README.md) — the Russian version is the primary one; this is a translation.

Bare-metal example programs for the K1921VG1T microcontroller (JSC "NIIET", RISC-V SCR5 core,
RV32IMAFDCP) in C and in Rust (`no_std`), a Rust peripheral access crate (PAC) generated from the
vendor's SVD description after fixing it, and an analysis of the defects found in that description.
The work was done within the DEVBOARDS programme of the RISC-V Alliance (Russia).

## Contents

| Path | What it is |
|---|---|
| [`projects/k1921vg1t-pac/`](projects/k1921vg1t-pac/) | PAC for K1921VG1T, the fixed SVD and the [SVD defect analysis](projects/k1921vg1t-pac/SVD_FIXES.en.md) |
| `projects/rust_*/` | 9 Rust examples, from blinking an LED to GOST encryption and the PLL |
| `projects/run_leds/`, `projects/binary_counter/`, `projects/led_breath/` | C examples, built with the vendor SDK |
| [`openocd/`](openocd/) | OpenOCD configurations for the FT2232HL Board adapter and K1921VG1T |
| [`Taskfile.yml`](Taskfile.yml) | build and flash commands for the Rust examples ([Task](https://taskfile.dev)) |
| [`NOTICE.md`](NOTICE.md), [`LICENSE`](LICENSE), `LICENSES/` | licenses |

## Rust examples

| Step | Project | What it does |
|---|---|---|
| 1 | `rust_blink` | blinks LED1 with no crates at all: own startup code and linker script |
| 2 | `rust_blink_rt` | the same blink on the `riscv-rt` runtime |
| 3 | `rust_blink_pac` | the same blink through the PAC: registers by name instead of by address |
| 4 | `rust_timer_irq` | toggles LED1 from the TMR0 interrupt (TMR0 → PLIC → machine external interrupt) |
| 5 | `rust_uart` | text output over UART0 (9600 8N1) and measurement of the internal HSI oscillator frequency |
| 6 | `rust_adc` | thermometer: the ADC measures the on-chip temperature sensor, output over UART |
| 7 | `rust_dac` | signal generator on DAC0 (64-point sine), output on the SMA connector "DAC 0" |
| 8 | `rust_crypto` | hardware "Kuznyechik" encryption with the CRYPTO block, checked against the GOST R 34.12-2015 test vector |
| 9 | `rust_pll` | switches the system clock to PLL0 (27 MHz crystal → 150 MHz) and measures it |

Steps 3–9 use the [PAC](projects/k1921vg1t-pac/). The examples are built for the
`riscv32imac-unknown-none-elf` target.

## C examples

| Project | What it does |
|---|---|
| `run_leds` | modified copy of the `run_leds` example from the NIIET SDK (the changes are listed at the top of `app/main.c`); `printf` messages go to UART0 at 115200 baud (SDK retarget setting) |
| `binary_counter` | counts presses of the USER BTN button (falling edge on pin B3, no debouncing) and shows the number 0–15 in binary on LED1–LED4; drives the GPIO registers directly |
| `led_breath` | smoothly raises and lowers the brightness of LED1 with software PWM (on-time proportional to the square of the step); LED2 is lit while the brightness rises |

`binary_counter` has a second build variant, `Makefile_fit`, in which the SDK source files are listed
explicitly instead of being picked by a wildcard.

## Hardware

- NIIET development board with K1921VG1T (КФДЛ.441461.050 rev 1.1; `NIIET-DEV-K1921VG1T` in the SDK).
  12 V power through the XS5 jack. Jumper XP5 "LEDS ON" must be fitted, otherwise
  LED1–LED4 are not connected to pins C12–C15.
- JTAG adapter [FT2232HL Board V3.6](https://github.com/arm8686/FT2232HL-Board). Its JTAG is wired to
  channel B (Interface 1). On Windows, install the WinUSB driver for Interface 1 with
  [Zadig](https://zadig.akeo.ie/); leave Interface 0 alone.
- For UART output: a USB-C cable to the board's XS4 "USB-UART" connector (CH340B bridge, UART0 of the
  chip).

Adapter connection to the board's XP7 "JTAG" header (straight, TDI/TDO not crossed):

| Signal | FT2232HL Board, CN4 | Board, XP7 |
|---|---|---|
| TMS | 7 | 2 |
| TCK | 9 | 4 |
| TDO | 13 | 6 |
| TDI | 5 | 8 |
| nRST | 15 | 10 |
| GND | GND | 3, 5 or 9 |

XP7 pin 1 (+3.3 V) is not connected to the adapter.

## Software

| What | Where | Tested with |
|---|---|---|
| Rust | [rustup](https://rustup.rs/), target `riscv32imac-unknown-none-elf` | 1.97.1 |
| NIIET OpenOCD | [gitflic.ru/project/niiet/openocd](https://gitflic.ru/project/niiet/openocd) (sources and releases) | `-00247-g90632ca40-dirty (2025-08-01-14:40)` |
| Task (optional) | [taskfile.dev](https://taskfile.dev/) | 3.54.0 (3.38 or newer required) |
| For C: NIIET SDK | [gitflic.ru/project/niiet/k1921vg1t_sdk](https://gitflic.ru/project/niiet/k1921vg1t_sdk) | commit `9fa40f85be5b` |
| For C: compiler | [xPack riscv-none-elf-gcc](https://github.com/xpack-dev-tools/riscv-none-elf-gcc-xpack/releases) | 13.4.0-1 |
| For C: make | GNU make (the `clean` rule expects the Windows command prompt) | GNU Make 3.81 |

The NIIET OpenOCD is required: the configuration uses the `k1921vg1t` flash driver from this fork.
Documentation on the microcontroller (user manual, errata, in Russian) is in the
[gitflic.ru/project/niiet/niiet](https://gitflic.ru/project/niiet/niiet) repository.

## Quick start

Run the commands from the repository root.

1. Install Rust and the target:

   ```sh
   rustup target add riscv32imac-unknown-none-elf
   ```

2. Connect the adapter and the board, power on. Check the connection to the chip:

   ```sh
   task scan
   ```

   or without Task:

   ```sh
   openocd -s openocd -f ft2232hl_board.cfg -f k1921vg1t_singlecore_dev.cfg -c init -c shutdown
   ```

   Signs of success in the output: `0xdeb15001`, `Examined RISC-V core; found 1 harts`,
   `hart 0: XLEN=32, misa=0x4014912d`.

3. Build and flash an example:

   ```sh
   task run P=rust_blink_pac
   ```

   If `openocd` is not on `PATH`, pass its location:
   `task run P=rust_blink_pac OPENOCD=<path>/openocd.exe`.

   Without Task the same is done with:

   ```sh
   cd projects/rust_blink_pac && cargo build --release && cd ../..
   openocd -s openocd -f connect_ft2232_jtag.cfg -c halt \
     -c "flash write_image erase projects/rust_blink_pac/target/riscv32imac-unknown-none-elf/release/rust_blink_pac" \
     -c "verify_image projects/rust_blink_pac/target/riscv32imac-unknown-none-elf/release/rust_blink_pac" \
     -c shutdown
   ```

   Then press the RESET button (SB2) on the board. After the reset LED1 blinks.

4. For the examples with UART output (steps 5, 6, 8, 9) open a terminal on the COM port of the CH340B
   bridge: 9600 baud, 8 data bits, no parity, 1 stop bit.

## Building the C examples

Clone the NIIET SDK next to this repository, not inside it (this is where the Makefiles look for it by
default). From the repository root:

```sh
git clone https://gitflic.ru/project/niiet/k1921vg1t_sdk.git ../k1921vg1t_sdk
git -C ../k1921vg1t_sdk checkout 9fa40f85be5b
```

Build (without `COMPILER_PATH` the compiler is taken from `PATH`):

```sh
make -C projects/run_leds COMPILER_PATH=<path to xpack>/bin/
```

If the SDK is located elsewhere, pass its path with `SDK_DIR` (an absolute path or a path relative to
the project directory):

```sh
make -C projects/run_leds SDK_DIR=<path to SDK>
```

The built firmware is `projects/run_leds/build/run_leds.elf` (`build_fit/` for `Makefile_fit`). It is flashed the same way as in step 3:

```sh
openocd -s openocd -f connect_ft2232_jtag.cfg -c halt \
  -c "flash write_image erase projects/run_leds/build/run_leds.elf" \
  -c "verify_image projects/run_leds/build/run_leds.elf" -c shutdown
```

Then press the RESET button (SB2) on the board.

## Notes

- The TCK frequency for K1921VG1T must not exceed 200–250 kHz (NIIET document "Quick start with the
  K1921VG015, K1921VG1T… microcontrollers", in Russian, p. 5); the repository configurations use 200 kHz.
- The OpenOCD configuration has no reset control (`reset_config none`), so the board has to be reset
  with the SB2 button after flashing.
- The C examples were tested with the `k1921vg1t_sdk` SDK at commit `9fa40f85be5b`. Compatibility with
  newer SDK versions and with the vendor's new SDK
  [riscv_cmake_sdk](https://gitflic.ru/project/niiet/riscv_cmake_sdk) was not checked.
- In SDK version `9fa40f85be5b` the PLL setup block for the 27 MHz crystal is labelled
  "FOUT = 204 000 000 Hz", while its dividers give 150 MHz; in later SDK versions the label was
  corrected to 150 MHz. The `rust_pll` example sets PLL0 to 150 MHz.
- In `k1921vg1t_sdk` at commit `9fa40f85be5b` the OpenOCD configuration specifies JTAG ID `0xDEB05001`
  and 2000 kHz; the configuration in this repository uses ID `0xDEB15001`, read from the chip, and
  200 kHz. In later SDK versions the ID was corrected.

## License

© 2026 Valentin Chernykh (Черных В.А.). Unless stated otherwise, the materials in this repository are
licensed under [CC BY 4.0](https://creativecommons.org/licenses/by/4.0/legalcode). Third-party files
(the NIIET SVD description and example, svd2rust template code, OpenOCD configurations) are under
their own licenses — see [NOTICE.md](NOTICE.md).
