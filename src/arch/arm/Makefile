GCC_PREFIX_2    = $(GCC_PREFIX)arm-none-eabi-

TARGET         ?= arm-unknown-linux-gnueabi

LDFLAGS        ?=

LLCFLAGS       ?= -mtriple=$(TARGET) $(MAYBE_OPTIMIZE) \
	-march=arm -mcpu=arm926ej-s --float-abi=hard -asm-verbose

QEMU           ?= qemu-system-arm

OBJS           ?= $(BDIR)/loader.o $(BDIR)/aeabi_runtime.o $(BDIR)/main.o
LINK           ?= $(BDIR)/linker.ld $(OBJS) $(BDIR)/initram.elf.embed
LIBS           ?=

DEP_RM         ?= arch/
DEP_KEEP       ?= arch/arm

LD             = $(GCC_PREFIX_2)ld

include ../../common/Makefile

.PHONY: all run debug

all: $(BDIR)/kernel.bin
	@wc -c $^

%.s: %.bc
	$(LLC) $(LLCFLAGS) $^ -o $@

# Assemble object files. note: do not overwrite deps with -MD $*.d
%.o: %.s
	$(AS) -g $< -o $@

# initram
$(BDIR)/%.o: %.s
	$(AS) $< -o $@

# kernel (object)
$(BDIR)/kernel.elf: $(LINK)
	$(LD) -o $@ -T $^

# kernel (binary image)
$(BDIR)/kernel.bin: $(BDIR)/kernel.elf
	$(OBJCOPY) -O binary $^ $@

# running
run: all
	$(QEMU) -M versatilepb -m 32M -nographic -kernel $(BDIR)/kernel.bin

debug: $(BDIR)/kernel.elf
ifeq ($(strip $(TMUX)),)
	tmux new-session -d -s rustboot
	tmux new-window -t rustboot:1 "$(QEMU) -M versatilepb -m 32M -nographic -kernel $^ -s -S"
	tmux split-window -t rustboot "$(GDB)"
	tmux a -t rustboot
	tmux kill-session -t rustboot
else
	# TODO: debug in current window, can't kill panes
	tmux new-w "$(QEMU) -M versatilepb -m 32M -nographic -kernel $^ -s -S"
	tmux split-w "$(GDB); tmux kill-w"
endif
