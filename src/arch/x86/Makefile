
# localized cross-compiled binutils
TARGET         ?= i686-unknown-linux-gnu
GCC_TARGET      = i386-elf

RUSTCFLAGS_2   ?=

NASMFLAGS      ?= -g -f elf32
LLCFLAGS       ?=

LDFLAGS        ?= -melf_i386 --gc-sections
LDFLAGS_EMBED  ?= -melf_i386

QEMU           ?= qemu-system-i386

include src/Makefile


all: $(BDIR)/floppy.img
	@wc -c $^

# running
run: all
	$(QEMU) -fda $(BDIR)/floppy.img

# create Floppy Disk image with MBR by concatenating boot & kernel
SECTIONS        = .text .data .rodata
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
