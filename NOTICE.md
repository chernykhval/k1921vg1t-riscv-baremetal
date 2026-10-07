# Уведомление о лицензиях / Notice

*English version below.*

## Лицензия этой работы

© 2026 Черных В.А. (Valentin Chernykh).

Если ниже не указано иное, все материалы репозитория — исходный код, конфигурационные файлы
и документация — распространяются на условиях лицензии
**Creative Commons «Атрибуция» 4.0 Международная (CC BY 4.0)**.

- Текст лицензии на русском: https://creativecommons.org/licenses/by/4.0/legalcode.ru
- Официальный текст на английском: [`LICENSE`](LICENSE)

Выбор CC BY 4.0 и для исходного кода обусловлен требованием Конкурса учебно-методических и
исследовательских работ Российского Альянса RISC-V.

## Сторонние материалы

Для изменённых чужих файлов действует правило: **оригинал — под лицензией правообладателя,
наши изменения — под CC BY 4.0.**

Текст открытой лицензии АО «НИИЭТ» (файл `LICENSE` из SDK `k1921vg1t_sdk`, коммит `9fa40f85be5b`)
приведён дословно в [`LICENSES/LicenseRef-NIIET-Open-License.txt`](LICENSES/LicenseRef-NIIET-Open-License.txt);
в `projects/k1921vg1t-pac/Cargo.toml` она обозначена как `LicenseRef-NIIET-Open-License`.

| Файлы | Происхождение | Лицензия |
|---|---|---|
| `projects/k1921vg1t-pac/K1921VG1T.patched.svd`, `projects/k1921vg1t-pac/K1921VG1T.svd.diff` | Изменённая копия `tools/svd/K1921VG1T.svd` и перечень изменений к нему из SDK НИИЭТ [`k1921vg1t_sdk`](https://gitflic.ru/project/niiet/k1921vg1t_sdk), коммит `9fa40f85be5ba72c1e76f9761f1c57e9b4b282ae`. Описание изменений — в [`SVD_FIXES.md`](projects/k1921vg1t-pac/SVD_FIXES.md). | Оригинал — [открытая лицензия АО «НИИЭТ»](https://gitflic.ru/project/niiet/k1921vg1t_sdk/blob?file=LICENSE&branch=master) (© 2025 АО «НИИЭТ»); изменения — CC BY 4.0 |
| `projects/k1921vg1t-pac/src/**` (кроме двух файлов ниже), `projects/k1921vg1t-pac/device.x` | Сгенерированы [svd2rust](https://github.com/rust-embedded/svd2rust) 0.37.1 из изменённого SVD, без ручных правок. | Производные от SVD: открытая лицензия АО «НИИЭТ»; в части, возникшей из наших изменений SVD, — CC BY 4.0 |
| `projects/k1921vg1t-pac/src/generic.rs`, `projects/k1921vg1t-pac/src/generic/raw.rs`, `projects/k1921vg1t-pac/build.rs` | Шаблонный код, который svd2rust 0.37.1 добавляет в каждый сгенерированный крейт. | MIT ([`LICENSES/MIT.txt`](LICENSES/MIT.txt)) или Apache-2.0 ([`LICENSES/Apache-2.0.txt`](LICENSES/Apache-2.0.txt)) на выбор; Copyright (c) 2016 Jorge Aparicio |
| `projects/run_leds/app/main.c` | Изменённая копия примера `run_leds` из SDK НИИЭТ (автор — А. Дыхно, АО «НИИЭТ»). | Оригинал — открытая лицензия АО «НИИЭТ»; изменения — CC BY 4.0 |
| `projects/run_leds/Makefile`, `projects/binary_counter/Makefile`, `projects/binary_counter/Makefile_fit`, `projects/led_breath/Makefile` | Makefile `run_leds` написан на основе `platform/Device/K1921VG015/gcc/makefile` из SDK НИИЭТ [`niiet_riscv_sdk`](https://gitflic.ru/project/niiet/niiet_riscv_sdk) (коммит `f968c4dc48b3`); остальные три — производные от него (напрямую или через Makefile `binary_counter`). | Оригинал — открытая лицензия АО «НИИЭТ»; изменения — CC BY 4.0 |
| `openocd/k1921vg1t_singlecore_dev.cfg` | Изменённая копия `tools/openocd/scripts/target/k1921vg1t_singlecore.cfg` из SDK НИИЭТ. | Оригинал — открытая лицензия АО «НИИЭТ»; изменения — CC BY 4.0 |
| `openocd/ft2232hl_board.cfg` | Доработка конфигурации OpenOCD из руководства к [FT2232HL Board](https://github.com/arm8686/FT2232HL-Board): синтаксис обновлён под современный OpenOCD, значения `layout_init` и сигналов взяты без изменений. | Оригинал — Apache-2.0, лицензия репозитория arm8686/FT2232HL-Board ([`LICENSES/Apache-2.0.txt`](LICENSES/Apache-2.0.txt)); изменения — CC BY 4.0 |

## Что не входит в репозиторий

Документация производителей (руководство пользователя, errata, схемы и руководства плат,
спецификации RISC-V), SDK НИИЭТ, сборка OpenOCD НИИЭТ и тулчейны в репозитории **не**
распространяются. Ссылки для загрузки — в [`README.md`](README.md).

---

# Notice (English)

## License of this work

Copyright © 2026 Valentin Chernykh (Черных В.А.).

Unless stated otherwise below, all materials in this repository — source code, configuration
files and documentation — are licensed under the
**Creative Commons Attribution 4.0 International License (CC BY 4.0)**
([`LICENSE`](LICENSE), https://creativecommons.org/licenses/by/4.0/legalcode).

CC BY 4.0 is used for the source code as well because the rules of the RISC-V Alliance (Russia)
contest of educational and research works require it.

## Third-party material

For modified third-party files: **the original is under its owner's license, our
modifications are under CC BY 4.0.**

The text of the NIIET open license (the `LICENSE` file of the `k1921vg1t_sdk` SDK, commit
`9fa40f85be5b`, in Russian) is reproduced verbatim in
[`LICENSES/LicenseRef-NIIET-Open-License.txt`](LICENSES/LicenseRef-NIIET-Open-License.txt);
`projects/k1921vg1t-pac/Cargo.toml` refers to it as `LicenseRef-NIIET-Open-License`.

| Files | Origin | License |
|---|---|---|
| `projects/k1921vg1t-pac/K1921VG1T.patched.svd`, `projects/k1921vg1t-pac/K1921VG1T.svd.diff` | Modified copy of `tools/svd/K1921VG1T.svd` and the list of changes to it from the NIIET SDK [`k1921vg1t_sdk`](https://gitflic.ru/project/niiet/k1921vg1t_sdk), commit `9fa40f85be5ba72c1e76f9761f1c57e9b4b282ae`. Changes are described in [`SVD_FIXES.md`](projects/k1921vg1t-pac/SVD_FIXES.md). | Original: NIIET open license ([LICENSE in the SDK](https://gitflic.ru/project/niiet/k1921vg1t_sdk/blob?file=LICENSE&branch=master), © 2025 JSC "NIIET"); modifications: CC BY 4.0 |
| `projects/k1921vg1t-pac/src/**` (except the two files below), `projects/k1921vg1t-pac/device.x` | Generated by [svd2rust](https://github.com/rust-embedded/svd2rust) 0.37.1 from the modified SVD, without manual edits. | Derived from the SVD: NIIET open license; the parts resulting from our SVD changes: CC BY 4.0 |
| `projects/k1921vg1t-pac/src/generic.rs`, `projects/k1921vg1t-pac/src/generic/raw.rs`, `projects/k1921vg1t-pac/build.rs` | Template code emitted by svd2rust 0.37.1 into every generated crate. | MIT ([`LICENSES/MIT.txt`](LICENSES/MIT.txt)) or Apache-2.0 ([`LICENSES/Apache-2.0.txt`](LICENSES/Apache-2.0.txt)), at your option; Copyright (c) 2016 Jorge Aparicio |
| `projects/run_leds/app/main.c` | Modified copy of the `run_leds` example from the NIIET SDK (author: A. Dykhno, NIIET). | Original: NIIET open license; modifications: CC BY 4.0 |
| `projects/run_leds/Makefile`, `projects/binary_counter/Makefile`, `projects/binary_counter/Makefile_fit`, `projects/led_breath/Makefile` | The `run_leds` Makefile is based on `platform/Device/K1921VG015/gcc/makefile` from the NIIET SDK [`niiet_riscv_sdk`](https://gitflic.ru/project/niiet/niiet_riscv_sdk) (commit `f968c4dc48b3`); the other three are derived from it (directly or via the `binary_counter` Makefile). | Original: NIIET open license; modifications: CC BY 4.0 |
| `openocd/k1921vg1t_singlecore_dev.cfg` | Modified copy of `tools/openocd/scripts/target/k1921vg1t_singlecore.cfg` from the NIIET SDK. | Original: NIIET open license; modifications: CC BY 4.0 |
| `openocd/ft2232hl_board.cfg` | Adapted from the OpenOCD configuration in the [FT2232HL Board](https://github.com/arm8686/FT2232HL-Board) manual: syntax updated for current OpenOCD, `layout_init` and signal values kept unchanged. | Original: Apache-2.0, the license of the arm8686/FT2232HL-Board repository ([`LICENSES/Apache-2.0.txt`](LICENSES/Apache-2.0.txt)); modifications: CC BY 4.0 |

## Not included

Vendor documentation (user manual, errata, board schematics and manuals, RISC-V
specifications), the NIIET SDK, the NIIET OpenOCD build and the toolchains are **not**
redistributed in this repository. See [`README.md`](README.md) for download links.
