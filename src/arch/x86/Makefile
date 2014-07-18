# need to create new variable due to semantics of make "recursive variables"
GCC_PREFIX_2    = $(GCC_PREFIX)i386-elf-

include ../../common/Makefile

-include $(BDIR)/loader.d

TARGET         ?= i686-unknown-linux-gnu

LDFLAGS        ?= -melf_i386 --gc-sections
LDFLAGS_EMBED  ?= -melf_i386

ASM            ?= nasm
ASMFLAGS       ?= -g -f elf32

QEMU           ?= qemu-system-i386

OBJS           ?= $(BDIR)/loader.o $(BDIR)/main.o
LINK           ?= $(BDIR)/linker.ld $(OBJS) $(BDIR)/initram.elf.embed
LIBS           ?=

SECTIONS       ?= .text .data .rodata

DEP_RM         ?= arch/
DEP_KEEP       ?= arch/i686\|arch/common


.PHONY: all run debug

all: $(BDIR)/floppy.img
	@wc -c $^

%.o: %.bc
	$(CC) $(CFLAGS) -c $^ -o $@

# initram
$(BDIR)/%.o: %.asm
	$(ASM) $(ASMFLAGS) -MD $(BDIR)/$*.d -o $@ $<

# Assemble loader
%.o: %.asm
	$(ASM) $(ASMFLAGS) -MD $*.d -o $@ $<

# kernel (object)
$(BDIR)/kernel.elf: $(LINK)
	$(LD) $(LDFLAGS) -o $@ -T $^ "-(" $(LIBS) "-)" -Map=./$(BDIR)/linker.map

# bootloader and kernel separately
$(BDIR)/kernel.bin: $(BDIR)/kernel.elf
	$(OBJCOPY) -O binary $(addprefix -j ,$(SECTIONS)) $^ $@

$(BDIR)/boot.bin: $(BDIR)/kernel.elf
	$(OBJCOPY) -O binary -j .boot $^ $@

# join both
$(BDIR)/floppy.img: $(BDIR)/boot.bin $(BDIR)/kernel.bin
	cat $^ > $@

# running
run: all
	$(QEMU) -fda $(BDIR)/floppy.img

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
