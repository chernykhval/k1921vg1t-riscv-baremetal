#==============================================================================
# Makefile: run_leds for K1921VG1T (NIIET-DEV-K1921VG1T board)
#------------------------------------------------------------------------------
# Adapted from the NIIET makefile niiet_riscv_sdk/platform/Device/K1921VG015/gcc/makefile:
#   * xPack prefix riscv-none-elf- instead of riscv64-unknown-elf- (sc-dt);
#   * linking through gcc, so multilib, newlib and libgcc are selected
#     automatically;
#   * march/mabi taken from the NIIET run_leds project; RETARGET added (printf
#     over UART0), HSECLK_VAL set to the 27 MHz crystal of this board.
# The application is a local copy in ./app (the SDK itself is not modified).
#
# Build: make            Clean: make clean
# Toolchain and SDK locations can be overridden on the command line or via
# environment variables:
#   make COMPILER_PATH=<path-to-gcc-bin>/ SDK_DIR=<path-to-k1921vg1t_sdk>
#==============================================================================

TARGET_NAME = run_leds

#--- Toolchain ----------------------------------------------------------------
# Empty COMPILER_PATH: riscv-none-elf-gcc is taken from PATH.
COMPILER_PATH ?=
CROSS   = $(COMPILER_PATH)riscv-none-elf-
CC      = $(CROSS)gcc
OBJCOPY = $(CROSS)objcopy
OBJDUMP = $(CROSS)objdump
SIZE    = $(CROSS)size

#--- SDK paths ----------------------------------------------------------------
# Default: k1921vg1t_sdk cloned next to this repository.
SDK_DIR   ?= ../../../k1921vg1t_sdk
DEV_DIR   = $(SDK_DIR)/platform/Device/K1921VG1T
APP_DIR   = ./app
RETGT_DIR = $(SDK_DIR)/platform/retarget/Template/K1921VG1T
BSP_DIR   = $(SDK_DIR)/hardware/bsp/NIIET-DEV-K1921VG1T

BUILD_DIR = build

#--- Sources ------------------------------------------------------------------
# All SDK sources are compiled; unused code is removed by --gc-sections.
CSRC = $(wildcard $(APP_DIR)/*.c)          \
       $(wildcard $(DEV_DIR)/source/*.c)   \
       $(wildcard $(RETGT_DIR)/*.c)        \
       $(wildcard $(BSP_DIR)/*.c)
ASRC = $(wildcard $(DEV_DIR)/source/*.S)

OBJS  = $(addprefix $(BUILD_DIR)/,$(notdir $(CSRC:.c=.o)) $(notdir $(ASRC:.S=.o)))
VPATH = $(APP_DIR) $(DEV_DIR)/source $(RETGT_DIR) $(BSP_DIR)

#--- Flags --------------------------------------------------------------------
MARCH   = rv32imafdc_zicsr_zifencei
MABI    = ilp32d
# HSECLK_VAL: the board crystal (BQ2) is 27 MHz.
DEFINES = -DRETARGET -DUSE_LIBC -DSYSCLK_HSE -DCKO_NONE -DHSECLK_VAL=27000000
INCS    = -I$(DEV_DIR)/Include -I$(RETGT_DIR) -I$(BSP_DIR)

# Flash image; use k1921vg1t_ram.ld to run from RAM.
LDSCRIPT = $(DEV_DIR)/ldscripts/k1921vg1t_flash.ld

CFLAGS  = -march=$(MARCH) -mabi=$(MABI) -Og -g -Wall -std=gnu99 \
          -ffunction-sections -fdata-sections $(DEFINES) $(INCS)

ASFLAGS = -march=$(MARCH) -mabi=$(MABI) -g $(DEFINES) $(INCS)

# -nostartfiles: the SDK startup_k1921vg1t.S replaces crt0.
# -L ldscripts:  k1921vg1t_flash.ld includes k1921vg1t_common.lds.
LDFLAGS = -march=$(MARCH) -mabi=$(MABI) -nostartfiles \
          -T $(LDSCRIPT) -L$(DEV_DIR)/ldscripts \
          -Wl,--gc-sections -Wl,-Map=$(BUILD_DIR)/$(TARGET_NAME).map

ELF = $(BUILD_DIR)/$(TARGET_NAME).elf
HEX = $(BUILD_DIR)/$(TARGET_NAME).hex
LST = $(BUILD_DIR)/$(TARGET_NAME).lst

#--- Rules --------------------------------------------------------------------
all: $(ELF) $(HEX) $(LST) size

$(BUILD_DIR):
	-mkdir $(BUILD_DIR)

$(BUILD_DIR)/%.o: %.c | $(BUILD_DIR)
	@echo COMPILE $<
	$(CC) $(CFLAGS) -c $< -o $@

$(BUILD_DIR)/%.o: %.S | $(BUILD_DIR)
	@echo ASSEMBLE $<
	$(CC) $(ASFLAGS) -c $< -o $@

$(ELF): $(OBJS)
	@echo LINK $@
	$(CC) $(OBJS) $(LDFLAGS) -o $@

$(HEX): $(ELF)
	$(OBJCOPY) -O ihex $< $@

$(LST): $(ELF)
	$(OBJDUMP) -S $< > $@

size: $(ELF)
	$(SIZE) $(ELF)

clean:
	-rmdir /S /Q $(BUILD_DIR)

.PHONY: all size clean
