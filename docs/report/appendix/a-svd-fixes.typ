#import "../lib/template.typ": todo

= Приложение А. Места исправлений в SVD-описании <app-svd>

В приложении перечислены все 48 мест SVD-описания К1921ВГ1Т (SDK `k1921vg1t_sdk`, коммит
`9fa40f85be5b`), исправленных, чтобы из него можно было сгенерировать PAC. Номера классов
соответствуют #ref(<tab-svd-a>, supplement: [таблице])\; там же указано, что происходит без
исправления. Запись `[%s]` означает массив регистров или кластеров. Построчный перечень правок
приведён в файле `projects/k1921vg1t-pac/K1921VG1T.svd.diff`, обоснование каждой — в
`projects/k1921vg1t-pac/SVD_FIXES.md`.

#[
#set table(inset: (x: 4pt, y: 2.5pt))
#show table: set text(size: 9.5pt)
#show figure: set block(breakable: false)
#figure(
  table(
    columns: (auto, 1.7fr, 1fr, 1.3fr),
    align: (center, left, left, left),
    table.header([Класс], [Где], [Было #sym.arrow стало], [Основание]),
    [1], [блок `<cpu>`], [`Endian.LITTLE` #sym.arrow~`little`], [список допустимых значений в спецификации CMSIS-SVD],
    [2], [блок `<cpu>`], [пустой `<nvicPrioBits />` #sym.arrow `4`], [тег обязателен по схеме CMSIS-SVD; значение условное, в К1921ВГ1Т нет контроллера NVIC],
    [3], [единственный регистр в кластерах-массивах (30 мест): `GPIOA.MASKLB`, `GPIOA.MASKHB`, `CANFD0.MESSAGE`, `CAN.LIST`, `CAN.MSPND`, `CAN.MSID`, `ADC.CHCTL`, `ADC.CHDELAY`, `HASH.HR`, `CRYPTO.IV`, `CRYPTO.TEXT_IN`, `CRYPTO.KEY`, `CRYPTO.TEXT_OUT`, `FLASHM.DATA`, `FLASHM.ERRMASK`, `SIU.TMRMUX`, `RCU.UARTCFG`, `RCU.SPICFG`, `RCU.I2SCFG`, `RCU.USBCFG`, `RTC.GPR`, `TRNG.KEY`, `TRNG.FIFO`, `EMC.WINCFG`, `FLASHD.DATA`, `FLASHD.ERRMASK`, `LIN0.DTX`, `LIN0.DRX`, `I2C0.EADDR`, `MSC.U_ITEM` (у всех — `[%s]`)], [пустое имя #sym.arrow `VALUE`], [имя регистра обязательно; имя `VALUE` выбрано в работе],
    [4], [`CAN.Node[%s].Reserved`, `DMA.CH[%s].Reserved`, `DMA.Reserved`, `SIU.Reserved`, `SRAM.Reserved`, `SDRAM.Reserved`, `DAC0.Reserved`, `TMR0.CAPCOM[%s].Reserved`], [ширина 0 #sym.arrow 32~бита], [ширина регистров по умолчанию в этом файле; у `DAC0.Reserved` есть поля до бита 16],
    [5], [`ADC.ACTL[%s].<>TACC0`, `ADC.ACTL[%s].<>Reserved`], [префикс `<>` удалён], [допустимые символы имени по схеме CMSIS-SVD; соседний регистр называется `TACC1`],
    [6], [`ADC.SEQ[%s].SCVAL`, `HASH.SR`, `SIU.NODESTEST`, `MSC.U_ITEM[%s]`], [#box[`write-only`] #sym.arrow #box[`read-write`]], [в регистре есть и читаемые, и записываемые поля; РП, с. 744, 787, 581, 699],
    [7], [`TMR0.CAPCOM[%s]`], [шаг 12 #sym.arrow 16~байт], [элемент — четыре 32-битных регистра; РП, с. 673],
    [8], [регистр `ACTL` в кластере `ADC.ACTL[%s]`], [`ACTL` #sym.arrow `CTRL`], [совпадение имён кластера и регистра; схема имён svd2rust],
  ),
  caption: [Места исправлений в SVD-описании (48 мест)],
) <tab-svd-places>
]

Кроме этих правок, в исправленном файле устранены расхождения 9, 11 (частично) и 15 из
#ref(<tab-svd-b>, supplement: [таблицы])\; они разобраны в #ref(<sec-svd-defects>, supplement:
[подразделе]).
