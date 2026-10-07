# k1921vg1t-pac

[Русская версия](README.md) — the Russian version is the primary one; this is a translation.

Peripheral Access Crate (PAC) for the K1921VG1T microcontroller (JSC "NIIET", RISC-V), generated
with svd2rust from the SVD description in the vendor SDK. The SVD had to be fixed before generation.
The defects found and all fixes are described in [SVD_FIXES.en.md](SVD_FIXES.en.md).

The crate targets `no_std` programs for `riscv32imac-unknown-none-elf`. It is not published on
crates.io; use it as a path dependency.

## Contents

| File | Purpose |
|---|---|
| `K1921VG1T.patched.svd` | fixed SVD description, the source file for generation |
| `K1921VG1T.svd.diff` | all changes against the SDK SVD (`9fa40f85be5b`) |
| `SVD_FIXES.md`, `SVD_FIXES.en.md` | SVD defects found, grounds for the fixes, how they were checked (Russian and English) |
| `src/`, `build.rs`, `device.x` | svd2rust 0.37.1 output (split by form, formatted by rustfmt), no manual edits |
| `Cargo.toml` | crate manifest (written by hand) |

## Usage

```toml
[dependencies]
k1921vg1t-pac = { path = "../k1921vg1t-pac" }
```

A shortened excerpt of `rust_blink_pac` — enable the clock and release the reset of port GPIOC, drive
line C12 high:

```rust
use k1921vg1t_pac as pac;

let p = unsafe { pac::Peripherals::steal() };
p.rcu.cgcfgahb().modify(|_, w| w.gpiocen().set_bit());
p.rcu.rstdisahb().modify(|_, w| w.gpiocen().set_bit());
p.gpioc.outenset().write(|w| w.pin12().set_bit());
p.gpioc.dataout().write(|w| unsafe { w.val().bits(0x1000) });
```

Register arrays and clusters are accessed by index: `p.rcu.uartcfg(0).value()`, `p.adc.actl(3).ctrl()`.
The names `value` and `ctrl` were introduced when fixing the SVD (items 3 and 8 in SVD_FIXES).

## Notes

- `write()` starts from the register's reset value, `write_with_zero()` (unsafe) from zero. This
  matters for registers with a non-zero reset value. Example: `RCU.PLL[0].CFG` in `rust_pll`, whose
  reset value has the PLL power-down bit set.
- The PLIC interrupt controller is not described in the SVD, so the PAC does not have it. The
  `rust_timer_irq` example accesses it directly by address.
- The Cargo feature `rt` enables the `__EXTERNAL_INTERRUPTS` array and `device.x` for riscv-rt 0.13.
  The crate builds with it, but the repository examples do not use it and it was not tested on the
  board. The examples depend on riscv-rt 0.18 directly.
- The code was generated without an svd2rust RISC-V settings file, in the "legacy" interrupt
  rendering mode; svd2rust prints two warnings about it: `No settings file provided for RISC-V target.
  Using legacy interrupts rendering` and `Please, consider migrating your PAC to riscv 0.12.0 or later`.

## Regeneration

The commands that produce `src/` are given in [SVD_FIXES.en.md](SVD_FIXES.en.md#inputs). With the
same tool versions the result is byte-identical to the repository (checked on Windows).

## License

The crate combines material under different licenses: the NIIET SVD description (the rights
holder's open license) with the author's modifications (CC BY 4.0), code generated from it (NIIET
license and CC BY 4.0), and the svd2rust template code (MIT OR Apache-2.0). See
[NOTICE.md](../../NOTICE.md) for details.
