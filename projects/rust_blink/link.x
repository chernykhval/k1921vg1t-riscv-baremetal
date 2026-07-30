/* link.x — минимальный линкер-скрипт (аналог k1921vg1t_flash.ld из C).
 * Задаёт карту памяти и раскладку секций. Урезан до предела: наша мигалка
 * не использует инициализированных глобальных переменных, поэтому возни с
 * копированием .data из Flash в RAM (как в C-startup) здесь не нужно. */

OUTPUT_ARCH(riscv)
ENTRY(_start)               /* точка входа — метка _start в нашем ассемблере */

MEMORY {
  FLASH (rx)  : ORIGIN = 0x00000000, LENGTH = 1024K   /* программа */
  RAM   (rwx) : ORIGIN = 0x20000000, LENGTH = 512K    /* стек, данные */
}

/* Вершина стека — верхний край RAM. startup положит её в sp. */
_stack_top = ORIGIN(RAM) + LENGTH(RAM);

SECTIONS {
  /* Код. Секция .init (наш startup) идёт ПЕРВОЙ -> попадает на адрес 0x0,
   * откуда процессор стартует после сброса. */
  .text : {
    KEEP(*(.init))
    *(.text .text.*)
  } > FLASH

  /* Константы (только чтение) — тоже во Flash. */
  .rodata : {
    *(.rodata .rodata.*)
    *(.srodata .srodata.*)
  } > FLASH

  /* Неинициализированные данные — в RAM. Для мигалки пусто. */
  .bss (NOLOAD) : {
    *(.bss .bss.* .sbss .sbss.*)
    *(COMMON)
  } > RAM

  /* Отладочный/служебный мусор, который нам не нужен во Flash. */
  /DISCARD/ : {
    *(.eh_frame*)
    *(.comment)
  }
}
