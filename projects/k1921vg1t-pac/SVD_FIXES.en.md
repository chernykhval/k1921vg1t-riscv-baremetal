# Defects in the K1921VG1T SVD description and their fixes

[Русская версия](SVD_FIXES.md) — the Russian version is the primary one; this is a translation.

The Rust PAC `k1921vg1t-pac` is generated with svd2rust from the SVD description of the K1921VG1T
microcontroller shipped in the SDK of JSC "NIIET". svd2rust does not accept the original file. This
document lists every defect found: what was fixed, on what grounds, and what was left unchanged.

## Inputs

| | |
|---|---|
| Original SVD | `tools/svd/K1921VG1T.svd` from [k1921vg1t_sdk](https://gitflic.ru/project/niiet/k1921vg1t_sdk), commit `9fa40f85be5b` (2026-07-14), `<version>2.8</version>`, SHA-256 `56c48c5763a8948205322001c5e57b5ffa1b0655e537edd26d084e451553400c` |
| Fixed SVD | [`K1921VG1T.patched.svd`](K1921VG1T.patched.svd), SHA-256 `65bb7a392aafa3081d4b6a57572178269a6beee35672311321dacee3cbed9e29` |
| All changes | [`K1921VG1T.svd.diff`](K1921VG1T.svd.diff) — unified diff against the original file |
| Generator | svd2rust 0.37.1, form 0.13.0, rustfmt 1.9.0 |
| Reference documentation | K1921VG1T User Manual (UM_K1921VG1T_10YA, in Russian), "UM" below; page numbers are the printed ones |

The SHA-256 of the original file applies to the file extracted with `git show` (see below). A copy in
the SDK working tree on Windows may have CRLF line endings, and its checksum will differ.

Line numbers below refer to the **original** SDK file.

The fixed SVD is obtained from the original as follows:

```sh
git -C k1921vg1t_sdk show 9fa40f85be5b:tools/svd/K1921VG1T.svd > K1921VG1T.svd
patch -o K1921VG1T.patched.svd K1921VG1T.svd < K1921VG1T.svd.diff
```

The PAC is generated as follows (afterwards `src/` is byte-identical to the repository):

```sh
svd2rust --target riscv -i K1921VG1T.patched.svd
form -i lib.rs -o src/ && rm lib.rs
rustfmt --edition 2021 --config newline_style=Unix src/lib.rs build.rs
```

`newline_style=Unix` is needed for reproducibility: without it, rustfmt on Windows writes CRLF line
endings.

## Summary

**Group A — violations of the SVD standard and description errors that make the original file fail
generation or produce wrong code (8 classes, 48 places).** The exception is class 8: the standard
does not explicitly forbid such a description; the fix is needed because of svd2rust's naming scheme.
For each class it was checked that without its fix the PAC does not generate or does not compile:
only that class was reverted, all other fixes were kept. Within a class this does not hold for every
place: of the eight places of class 4 only one stops generation (item 4), and class 7 becomes
blocking only after the size of `TMR0.CAPCOM[%s].Reserved` is fixed (item 7).

| # | Defect | Places | Before | After | Without the fix |
|---|---|---|---|---|---|
| 1 | Invalid endianness value | 1 | `Endian.LITTLE` | `little` | svd2rust: `Unknown endianness` |
| 2 | Empty `nvicPrioBits` tag | 1 | `<nvicPrioBits />` | `<nvicPrioBits>4</nvicPrioBits>` | svd2rust: `Expected content in <nvicPrioBits> tag` |
| 3 | Registers without a name | 30 | `<name />` | `<name>VALUE</name>` | svd2rust: `Expected a <name> tag` |
| 4 | Zero-width registers | 8 | `<size>0</size>` | `<size>32</size>` | svd2rust: `Reset value 0x10102 doesn't fit in 0 bits` |
| 5 | `<>` characters in a register name | 2 | `<>TACC0`, `<>Reserved` | `TACC0`, `Reserved` | svd2rust panics: `"<>tacc0" is not a valid Ident` |
| 6 | `write-only` register with readable fields | 4 | `write-only` | `read-write` | svd2rust: `Field ADC.SEQ[%s].SCVAL.RCNT is not writtable or readable` |
| 7 | `TMR0.CAPCOM` array step smaller than the element | 1 | `<dimIncrement>12</dimIncrement>` | `16` | svd2rust: `Cluster CAPCOM[%s] has size 128 bits that is more then array increment 96 bits` |
| 8 | Cluster `ADC.ACTL` and its register share a name | 1 | register `ACTL` | `CTRL` | svd2rust passes, rustc: ``error[E0072]: recursive type `adc::actl::Actl` has infinite size`` |

Messages are quoted verbatim, including svd2rust's spelling (for class 8, rustc also reports
`error[E0428]: the name Actl is defined multiple times` together with E0072). svd2rust stops at the
first error, so the remaining places of each class were found by searching the file.

**Group B — defects found by checking the fixed SVD.** They do not block generation. Found by
comparing the SVD with the UM and with the `K1921VG1T.h` header from the same SDK.

| # | Defect | Decision |
|---|---|---|
| 9 | `addressBlock` of WDT and TMR0 is smaller than their registers occupy | fixed |
| 10 | DAC0 describes a register absent from the UM and from the SDK header | documented, not changed |
| 11 | Five interrupt sources from the UM interrupt table are missing in the SVD | 4 added, 1 documented |
| 12 | Status fields of HASH.SR are marked `write-only` | documented, not changed |
| 13 | TMR4–TMR15 inherit four CAPCOM blocks from TMR0; per the UM they have one | documented, not changed |
| 14 | Reset values of HASH.SR and MSC.U_ITEM disagree with the UM | documented, not changed |

**Group C — traces of an ARM example.** The file header matches the header of the `ARM_Example.svd`
example from ARM's CMSIS_5 package. Details below; only two fields were changed.

In total `K1921VG1T.svd.diff` contains the 48 places of group A, 6 changes of group B (two block
sizes and four interrupts) and 2 metadata fields of group C.

## Group A in detail

### 1. Endianness

- **Where:** line 24, `<cpu>` block.
- **Before / after:** `<endian>Endian.LITTLE</endian>` → `<endian>little</endian>`.
- **Grounds:** the CMSIS-SVD specification (`cpu` element) allows `little`, `big`, `selectable`,
  `other`. `Endian.LITTLE` is not among them.

### 2. nvicPrioBits tag

- **Where:** line 27, `<cpu>` block.
- **Before / after:** `<nvicPrioBits />` → `<nvicPrioBits>4</nvicPrioBits>`, in the same place: before
  `<vendorSystickConfig>`, as the element order of the CMSIS-SVD schema requires.
- **Grounds:** NVIC is the interrupt controller of Cortex-M cores. K1921VG1T has none; interrupts are
  handled by a PLIC. The tag belongs to Cortex-M cores (`ARM_Example.svd` has it with the value 3),
  but the CMSIS-SVD specification makes it mandatory inside `<cpu>`, and svd2rust requires it to
  contain a number: without the tag it reports `Expected a <nvicPrioBits> tag, found none`, with an
  empty tag — the error in the table above. The value 4 is nominal: in the PAC it only ends up in the
  `NVIC_PRIO_BITS` constant, which the crate code does not use.

### 3. Registers without a name (30 places)

- **Where:** lines 3880, 3909, 5320, 5652, 5702, 5731, 10703, 10746, 12207, 12259, 12288, 12317, 12642,
  17750, 18032, 20384, 23460, 23540, 23893, 23966, 24833, 25561, 26053, 26100, 27694, 27976, 34044,
  34073, 37076, 38114.
- **Before / after:** `<name />` → `<name>VALUE</name>`.
- **Grounds:** in all 30 cases this is the only register inside an array cluster, e.g.
  `GPIOA.MASKLB[%s]`, `RCU.UARTCFG[%s]`, `ADC.CHDELAY[%s]`, `CRYPTO.KEY[%s]`, `MSC.U_ITEM[%s]`. The
  cluster names the array; the register name is empty. CMSIS-SVD requires a register name. The name
  `VALUE` was chosen in this work; the UM does not use it. In the PAC the access looks like
  `rcu.uartcfg(0).value()`. The repository examples use such registers: `rcu.uartcfg(0).value()`
  configures the UART clock (`rust_uart`, `rust_adc`, `rust_crypto`, `rust_pll`), and
  `adc.chdelay(63).value()` sets an ADC channel delay (`rust_adc`).

### 4. Zero-width registers (8 places)

- **Where:** lines 6232 (`CAN.Node[%s].Reserved`), 16217 (`DMA.CH[%s].Reserved`), 17560 (`DMA.Reserved`),
  21511 (`SIU.Reserved`), 25305 (`SRAM.Reserved`), 26456 (`SDRAM.Reserved`), 32374 (`DAC0.Reserved`),
  33719 (`TMR0.CAPCOM[%s].Reserved`).
- **Before / after:** `<size>0</size>` → `<size>32</size>`.
- **Grounds:** seven of the eight registers are reserved slots with no fields, no description and a
  reset value of 0. The eighth, `DAC0.Reserved`, has fields up to bit 16 and the reset value
  `0x10102`, which is incompatible with a width of 0 bits; this is where svd2rust stops. svd2rust
  accepts the other seven places but generates wrong code: the registers get the type `u8`, and the
  `CAN.Node[%s]` array disappears (field `node: ()`). Checked by reverting these seven fixes. All eight
  were given a width of 32 bits, the default register size in this file. Each register's address is
  given explicitly (`addressOffset`), so fixing the size does not shift neighbouring registers.
  `DAC0.Reserved` is discussed separately in item 10.

### 5. `<>` characters in a name (2 places)

- **Where:** lines 10540 and 10683, cluster `ADC.ACTL[%s]`.
- **Before / after:** `<>TACC0` → `TACC0`, `<>Reserved` → `Reserved`. In XML the characters `<` and `>`
  are written as `&lt;` and `&gt;`, so the file contains `&lt;&gt;TACC0`, i.e. the name literally starts
  with `<>`.
- **Grounds:** a register name must match the `dimableIdentifierType` of the CMSIS-SVD XSD schema:
  Latin letters, digits, `_` and the `%s` placeholder are allowed. The characters `<>` are not. The
  neighbouring register of the same cluster is called `TACC1`, without the prefix.

### 6. "Write-only" register with readable fields (4 places)

- **Where:** lines 9926 (`ADC.SEQ[%s].SCVAL`), 12162 (`HASH.SR`), 21121 (`SIU.NODESTEST`),
  38118 (`MSC.U_ITEM[%s]`).
- **Before / after:** register `write-only` → `read-write`. Field access was not changed.
- **Grounds:** each of the four registers has both read-only and write-only fields. For example, in
  SCVAL the ICLR field is writable while ICNT and RCNT are readable. Hence the register is given
  `read-write` access. Checked against the UM: SCVAL — p. 744 (on the bit diagram ICLR is marked "з"
  (write), ICNT and RCNT "ч" (read)), NODESTEST — p. 581 (status bits STEx/STCx and commands
  CLR/START), U_ITEM — p. 699 (the CLEAR field clears the VALID bit by writing one, the other fields
  hold the received frame), HASH.SR — p. 787 (flags BUSY and DMAS are read; DCIS and DINIS are set by
  hardware and cleared, among other ways, by writing zero; for the access of the DCIS and DINIS fields
  themselves see item 12).

### 7. TMR0.CAPCOM array step

- **Where:** line 33606.
- **Before / after:** `<dimIncrement>12</dimIncrement>` → `16`.
- **Grounds:** an array element consists of four 32-bit registers (`CTRL`, `VAL0`, `VAL1`, `Reserved`
  at offset `0xC`), i.e. 16 bytes. With a step of 12 neighbouring elements overlap. Per the UM
  (p. 673) the blocks start at `+100h`, `+110h`, `+120h`, `+130h`, a step of 16 bytes.
- **Note:** in the original file the step of 12 does not contradict the zero width of
  `CAPCOM[%s].Reserved` (item 4): if both fixes are reverted together, neither svd2rust nor the
  compiler reports an error, and CAPCOM occupies `0x100–0x12F`. So the wrong step is revealed by
  checking against the UM, and by svd2rust only after the width is fixed.

### 8. Cluster and register with the same name

- **Where:** line 10415.
- **Before / after:** register `ACTL` inside cluster `ACTL[%s]` renamed to `CTRL`. The register
  description in the SVD is "ADC module control register".
- **Grounds:** in the `adc::actl` module svd2rust declares both the cluster struct `Actl` and the
  register type `Actl`. The names collide (E0428), and the field `actl: Actl` inside the struct
  refers to the struct itself (E0072). svd2rust does not notice; the compiler reports the errors. The
  CMSIS-SVD specification requires register names and, separately, cluster names to be unique within
  a peripheral; it does not explicitly forbid a cluster and a register nested in it from sharing a
  name. The fix is needed because of svd2rust's naming scheme. In the PAC the access looks like
  `adc.actl(3).ctrl()`.

## Group B in detail

### 9. addressBlock size (fixed)

- **WDT**, line 24864: the block is `0x1c` bytes, but registers `LOCK`/`LOCKSTAT` are at offset
  `0xC00` (UM, p. 807: watchdog lock register LOCK, offset +C00h). Changed to `0xc04`.
- **TMR0**, line 32850: the block is `0x130` bytes. That is exactly `0x100 + 4 × 12` and matches the
  wrong CAPCOM step from item 7. With a step of 16 the last CAPCOM block ends at `0x140`. Changed to
  `0x140`.
- Does not affect the generated code: checked by regenerating.

### 10. DAC0 register at offset 0x40 (not changed)

- **Where:** lines 32369–32374 and below.
- **What:** DAC0 describes a register `Reserved` with the description "MCP4921 control register" and
  fields `READY`, `BUF_VREF`, `NOT_GAIN`, `SCLK_DIV`. MCP4921 is a separate 12-bit DAC chip with an SPI
  interface made by Microchip ([datasheet DS22248A](https://ww1.microchip.com/downloads/en/DeviceDoc/22248a.pdf)).
  The descriptions of two of the four fields match the description of bits of the MCP4921 command
  word (DS22248A, p. 24, Register 5-1): field `BUF_VREF` — "VREF Input Buffer Control: 1 -
  buffered…", datasheet bit `BUF` — "VREF Input Buffer Control bit"; field `NOT_GAIN` — "1 - 1x … 0 -
  2x", datasheet bit `GA` — "1 = 1x … 0 = 2x".
- **Check:** in the UM the last register of the DAC block is `TEST` at `+28h` (p. 759). The UM has no
  register at `+40h` and no fields with these names. The `DAC_TypeDef` structure in `K1921VG1T.h` does
  not have it either. The register also lies outside the DAC0 `addressBlock` (`0x40` bytes). DAC1 is
  described as derived from DAC0 (`derivedFrom`) and inherits this register.
- **Decision:** the register was kept. Its absence from the documentation does not prove it is absent
  from the silicon. After fix 4 (width 0 → 32) it entered the PAC as `dac0.reserved()` (and
  `dac1.reserved()`). It should not be used. The repository examples do not access it.

### 11. Missing interrupts (4 added, 1 documented)

- **What:** the UM interrupt table (Table 13.1, pp. 187–193) and the `IsrVect_IRQ_*` enumeration in
  `K1921VG1T.h` contain five sources that are missing in the SVD (reserved numbers 168 and 169, also
  listed in the table, are not counted). Without them the PAC has no corresponding `Interrupt`
  variants and no handler names in `device.x`.
- **Added** (number and name per the UM, description from `K1921VG1T.h`):

  | Number | Name | SVD peripheral |
  |---|---|---|
  | 4 | `FLASH` | `FLASHM` |
  | 164 | `USB0` | `USBCTR0` |
  | 165 | `USB1` | `USBCTR1` |
  | 219 | `PWMx_TZ` | `PWM0` (per the UM, the combined interrupt of the PWM trip-zone detectors) |

  The SDK header calls the last one `PWMxTZ`, the UM `PWMx_TZ`. The UM name is used.
- **Not added:** 6 `L2_ERROR` (L2 cache error). In the SVD an interrupt is described inside a
  peripheral, and the SVD has no L2 cache peripheral; attaching it to another peripheral was not done.
- **Check after the fix:** the numbers of all 251 SVD interrupts match `K1921VG1T.h` and UM
  Table 13.1. The base addresses of all 106 peripherals match the `*_BASE` macros of the header and
  the UM memory map (Table 8.2, pp. 70–72).

### 12. Access of HASH.SR status fields (not changed)

- **Where:** lines 12182–12192.
- **What:** fields `DCIS` and `DINIS` are marked `write-only`. Per the UM (p. 787) they are flags set by
  hardware and cleared by software (among other ways, by writing zero). They must be read, but the PAC
  has no read method for them. In the UM itself, the SR register table labels these bits `DCIE` and
  `DINIE`, like the bits of the IMR register, although the descriptions are those of status bits.
- **Why not fixed:** a complete check of all field access against the UM was not done, so a single
  place was not fixed selectively.

### 13. Number of CAPCOM blocks in TMR4–TMR15 (not changed)

- **What:** all 15 timers TMR1–TMR15 are described in the SVD as derived from TMR0
  (`derivedFrom="TMR0"`) and inherit the `CAPCOM[0..3]` array. Per the UM (p. 220, translated):
  "TMR0-TMR3 have four identical CAPCOM capture/compare blocks. TMR4-TMR15 have one CAPCOM
  capture/compare block each." In the PAC, TMR4–TMR15 expose `capcom(1..3)`, which these timers do
  not have.
- **Why not fixed:** it requires a separate description of TMR4–TMR15 instead of inheritance. This
  changes the SVD structure and the PAC API and is out of scope. The `TMR_TypeDef` structure in the
  SDK header is also shared by all timers.

### 14. Reset values of HASH.SR and MSC.U_ITEM (not changed)

- **HASH.SR**, line 12163: the SVD has `<resetValue>0x1</resetValue>`, the UM (p. 787)
  "Сброс: 0000_0000h" (reset: 0000_0000h).
- **MSC.U_ITEM[%s]**: the SVD has reset value 0 for all elements, the UM (p. 699) "Сброс: 400h * i"
  (reset: 400h × i), i.e. each entry has its own value. A single `resetValue` for the elements of an
  SVD array cannot express that.
- **Why not fixed:** for HASH.SR the sources disagree; this can be checked by reading the register on
  the board after reset, which was not done. For MSC.U_ITEM, different reset values would require
  describing the array elements as separate registers; this was not done.

## Group C — traces of an ARM example

The header of the original file matches the header of the
[`ARM_Example.svd`](https://github.com/ARM-software/CMSIS_5/blob/develop/CMSIS/Utilities/ARM_Example.svd)
example from ARM's CMSIS_5 package: the same comments on the tags (e.g. "device vendor name",
"version of this description, adding CMSIS-SVD 1.1 tags"), and the `<licenseText>` text matches the
last five lines of that example's license. Values that belong to Cortex-M cores and to another NIIET
series: `<series>K1921VKx</series>` (the K1921VK series; for example, K1921VK01T is a microcontroller
"based on the ARM Cortex-M4F core" per NIIET's
[K1921VK01T quick start guide](https://niiet.ru/wp-content/uploads/%D0%91%D1%8B%D1%81%D1%82%D1%80%D1%8B%D0%B9_%D1%81%D1%82%D0%B0%D1%80%D1%82_%D0%9A1921%D0%92%D0%9A01%D0%A2.pdf),
in Russian, p. 3), a `<cpu>` block with `CM4`, `r0p1`, `mpuPresent`, `fpuPresent`, and the
`nvicPrioBits` tag. In `<licenseText>` the disclaimer is worded on behalf of ARM ("ARM SHALL NOT …
BE LIABLE").

- `series` changed to `K1921VGx`, core name `CM4` → `other`. These fields do not affect the code:
  checked by regenerating.
- `licenseText` is kept verbatim: changing legal text in someone else's file is outside the scope of
  this work. It does not reach the generated code.
- The remaining `<cpu>` fields and the description are unchanged.

## Fixes in later versions of the vendor SDK

In `k1921vg1t_sdk` the SVD file has not changed since commit `9fa40f85be5b` (checked at commit
`44510f7ad5e8`).

In the vendor's new SDK [riscv_cmake_sdk](https://gitflic.ru/project/niiet/riscv_cmake_sdk)
(file `tools/svd/K1921VG1T.svd`, checked at commit `ae5a8e69dfe0`) some of the described defects are
fixed: it has no defects of classes 1–5, the WDT block size is `0xc04` (item 9), the MCP4921 register
is removed from DAC0 (item 10), and the USB0, USB1 and PWMxTZ interrupts are added (item 11). Classes
6–8, the TMR0 block size `0x130`, the missing FLASH and L2_ERROR interrupts, and items 12–14 remain.
svd2rust does not accept this file: generation stops at the class 6 error.

## How it was checked

1. **Necessity of group A fixes.** For each of the 8 classes, an SVD with only that class reverted was
   built. svd2rust was run on it and, if it passed, `cargo check` on the generated crate. The results
   are in the summary table.
2. **Structural check of the fixed SVD** (by script): overlapping fields, fields beyond the register
   width, reset values outside fields, duplicate names, overlapping registers, registers outside the
   `addressBlock`, overlapping peripheral addresses, duplicate interrupt numbers, array steps.
   Remaining hits: 298 addresses holding more than one register (counting arrays and derived
   peripherals) — all declared via `<alternateRegister>`, in FLASHM and FLASHD as a chain of four
   registers; and the `+0x40` register of DAC0 and DAC1 outside the `addressBlock` (item 10).
3. **Comparison with the SDK header** `platform/Device/K1921VG1T/Include/K1921VG1T.h`: base addresses
   and interrupt numbers.
4. **Comparison with the UM** — items 6, 7, 9–14, plus the whole interrupt table 13.1 and memory
   map 8.2. The values in items 2, 3, 4 and 8 (the number 4, the name `VALUE`, the width 32, the name
   `CTRL`) were chosen by the author and not checked against the UM.

## Limitations

The fixed SVD is accepted by svd2rust and is consistent with the UM and the SDK header in the places
checked. The fixes were checked by generation, compilation and comparison with the documentation;
they were not specifically tested on the board. The file was not validated against the CMSIS-SVD XSD
schema. A complete check of all registers and fields against the UM was not done. The placeholder
names `VALUE` (item 3) do not appear in the UM. Defects 10, 12–14 and the `L2_ERROR` interrupt are
left as they are and described above.
