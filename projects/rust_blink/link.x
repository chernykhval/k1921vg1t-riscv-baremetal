/* Minimal linker script for K1921VG1T (counterpart of k1921vg1t_flash.ld from the SDK).
 * The program has no initialized globals, so .data is not copied from flash to RAM. */

OUTPUT_ARCH(riscv)
ENTRY(_start)

MEMORY {
  FLASH (rx)  : ORIGIN = 0x00000000, LENGTH = 1024K
  RAM   (rwx) : ORIGIN = 0x20000000, LENGTH = 512K
}

/* The stack starts at the top of RAM. */
_stack_top = ORIGIN(RAM) + LENGTH(RAM);

SECTIONS {
  /* .init goes first, at address 0x0, where the core starts after reset. */
  .text : {
    KEEP(*(.init))
    *(.text .text.*)
  } > FLASH

  .rodata : {
    *(.rodata .rodata.*)
    *(.srodata .srodata.*)
  } > FLASH

  .bss (NOLOAD) : {
    *(.bss .bss.* .sbss .sbss.*)
    *(COMMON)
  } > RAM

  /DISCARD/ : {
    *(.eh_frame*)
    *(.comment)
  }
}
