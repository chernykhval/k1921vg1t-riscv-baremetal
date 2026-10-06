#==============================================================================
# Makefile: led_breath для К1921ВГ1Т (плата NIIET-DEV-K1921VG1T)
#------------------------------------------------------------------------------
# Копия Makefile из binary_counter. Отличия:
#   * TARGET_NAME = led_breath
#   * APP_DIR = ./app
#
# Сборка:  make
# Очистка: make clean
#==============================================================================

TARGET_NAME = led_breath

#--- Тулчейн ------------------------------------------------------------------
COMPILER_PATH = C:/niiet/toolchain/xpack-riscv-none-elf-gcc-13.4.0-1/bin/
CROSS   = $(COMPILER_PATH)riscv-none-elf-
CC      = $(CROSS)gcc
OBJCOPY = $(CROSS)objcopy
OBJDUMP = $(CROSS)objdump
SIZE    = $(CROSS)size

#--- Пути к SDK ---------------------------------------------------------------
SDK_DIR   = ../../../k1921vg1t_sdk
DEV_DIR   = $(SDK_DIR)/platform/Device/K1921VG1T
APP_DIR   = ./app
RETGT_DIR = $(SDK_DIR)/platform/retarget/Template/K1921VG1T
BSP_DIR   = $(SDK_DIR)/hardware/bsp/NIIET-DEV-K1921VG1T

BUILD_DIR = build

#--- Исходники ----------------------------------------------------------------
CSRC = $(wildcard $(APP_DIR)/*.c)          \
       $(wildcard $(DEV_DIR)/source/*.c)   \
       $(wildcard $(RETGT_DIR)/*.c)        \
       $(wildcard $(BSP_DIR)/*.c)
ASRC = $(wildcard $(DEV_DIR)/source/*.S)

OBJS  = $(addprefix $(BUILD_DIR)/,$(notdir $(CSRC:.c=.o)) $(notdir $(ASRC:.S=.o)))
VPATH = $(APP_DIR) $(DEV_DIR)/source $(RETGT_DIR) $(BSP_DIR)

#--- Флаги --------------------------------------------------------------------
MARCH   = rv32imafdc_zicsr_zifencei
MABI    = ilp32d
DEFINES = -DUSE_LIBC -DSYSCLK_HSE -DCKO_NONE -DHSECLK_VAL=27000000
INCS    = -I$(DEV_DIR)/Include -I$(RETGT_DIR) -I$(BSP_DIR)

LDSCRIPT = $(DEV_DIR)/ldscripts/k1921vg1t_flash.ld

CFLAGS  = -march=$(MARCH) -mabi=$(MABI) -Og -g -Wall -std=gnu99 \
          -ffunction-sections -fdata-sections $(DEFINES) $(INCS)

ASFLAGS = -march=$(MARCH) -mabi=$(MABI) -g $(DEFINES) $(INCS)

LDFLAGS = -march=$(MARCH) -mabi=$(MABI) -nostartfiles \
          -T $(LDSCRIPT) -L$(DEV_DIR)/ldscripts \
          -Wl,--gc-sections -Wl,-Map=$(BUILD_DIR)/$(TARGET_NAME).map

ELF = $(BUILD_DIR)/$(TARGET_NAME).elf
HEX = $(BUILD_DIR)/$(TARGET_NAME).hex
LST = $(BUILD_DIR)/$(TARGET_NAME).lst

#--- Правила ------------------------------------------------------------------
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
