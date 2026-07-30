/* memory.x — карта памяти чипа для riscv-rt.
 * Мы даём ТОЛЬКО это; сам линкер-скрипт (link.x) предоставляет riscv-rt,
 * и он включает наш memory.x строкой INCLUDE memory.x. */

MEMORY
{
  FLASH : ORIGIN = 0x00000000, LENGTH = 1024K   /* программа */
  RAM   : ORIGIN = 0x20000000, LENGTH = 512K    /* данные, стек */
}

/* riscv-rt раскладывает секции по этим "регионам-псевдонимам".
 * Код и константы -> FLASH; изменяемые данные, стек, куча -> RAM. */
REGION_ALIAS("REGION_TEXT",   FLASH);
REGION_ALIAS("REGION_RODATA", FLASH);
REGION_ALIAS("REGION_DATA",   RAM);
REGION_ALIAS("REGION_BSS",    RAM);
REGION_ALIAS("REGION_HEAP",   RAM);
REGION_ALIAS("REGION_STACK",  RAM);
