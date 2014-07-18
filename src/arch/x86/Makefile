
# localized cross-compiled binutils
GCC_PREFIX_2    = $(GCC_PREFIX)i386-elf-

TARGET         ?= i686-unknown-linux-gnu

RUSTCFLAGS_2   ?=

CFLAGS         ?= -ffreestanding -target $(TARGET) $(MAYBE_CLANG_OPTIMIZE) \
	$(MAYBE_DEBUG) -fdata-sections -ffunction-sections

ASMFLAGS       ?= -g -f elf32

LDFLAGS        ?= -melf_i386 --gc-sections
LDFLAGS_EMBED  ?= -melf_i386

QEMU           ?= qemu-system-i386

OBJS           ?= $(BDIR)/loader.o $(BDIR)/main.o
LINK           ?= $(BOOTDIR)/linker.ld $(OBJS) $(BDIR)/initram.elf.embed
LIBS           ?=

SECTIONS       ?= .text .data .rodata

include src/Makefile


all: $(BDIR)/floppy.img
	@wc -c $^

# running
run: all
	$(QEMU) -fda $(BDIR)/floppy.img

# create Floppy Disk image with MBR by concatenating boot & kernel
$(BDIR)/kernel.bin: $(BDIR)/kernel.elf
	$(OBJCOPY) -O binary $(addprefix -j ,$(SECTIONS)) $^ $@
$(BDIR)/boot.bin: $(BDIR)/kernel.elf
	$(OBJCOPY) -O binary -j .boot $^ $@
$(BDIR)/floppy.img: $(BDIR)/boot.bin $(BDIR)/kernel.bin
	cat $^ > $@

debug: $(BDIR)/kernel.elf $(BDIR)/floppy.img
ifeq ($(strip $(TMUX)),)
	tmux new-session -d -s rustboot "$(QEMU) -fda $(BDIR)/floppy.img -m 32 -s -S"
	tmux new-window -t rustboot:1 "$(GDB)"
	tmux a -t rustboot
	tmux kill-session -t rustboot
else
	tmux split-w "$(GDB); tmux kill-p"
	$(QEMU) -fda $(BDIR)/floppy.img -m 32 -s -S
endif
