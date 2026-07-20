# К1921ВГ1Т — bare-metal (программа DEVBOARDS)

Учебный проект bare-metal (в перспективе — первый bare-metal **Rust**) для российского
RISC-V микроконтроллера **К1921ВГ1Т** (двухъядерный SCR5, RV32IMAFDCP).
Программатор — внешний **FT2232HL Board V3.6** (JTAG на канале B / Interface 1).
Полный план и контекст — в `reports/k1921vg1t_full_report.md`.

## Статус
**Ступень 3 пройдена:** OpenOCD видит чип по JTAG — tap `0xDEB15001`, ядро SCR5,
`misa=0x4014912d`. Дальше — Ступень 4 (прошивка `run_leds`).

## Структура
- `openocd/` — наши конфиги OpenOCD (интерфейс программатора + target).
- `reports/` — отчёт по проекту.
- `notes/`   — рабочие заметки.
- `docs/`    — документация (UM, Errata, схемы), собранная при исследовании.

## Проверить связь с чипом
```powershell
& "C:\niiet\sc-dt_Patch_Niiet_Win32\tools\bin\openocd.exe" `
  -f "C:\niiet\k1921vg1t-project\openocd\ft2232hl_board.cfg" `
  -f "C:\niiet\k1921vg1t-project\openocd\k1921vg1t_singlecore_dev.cfg" `
  -c "init" -c "shutdown"
```
Признак успеха: `tap/device found: 0xdeb15001`, `Examined RISC-V core; found 1 harts`,
`hart 0: XLEN=32, misa=0x4014912d`.
