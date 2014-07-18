
# localized cross-compiled binutils
GCC_PREFIX_2    = $(GCC_PREFIX)arm-none-eabi-

TARGET         ?= arm-unknown-linux-gnueabi

RUSTCFLAGS_2   ?= -C target-cpu=arm926ej-s

LLCFLAGS       ?= -mtriple=$(TARGET) $(MAYBE_OPTIMIZE) \
	-march=arm -mcpu=arm926ej-s --float-abi=hard -asm-verbose

ASMFLAGS       ?= -g

LDFLAGS        ?=
LDFLAGS_EMBED  ?=

QEMU           ?= qemu-system-arm

OBJS           ?= $(BDIR)/loader.o $(BDIR)/aeabi_runtime.o $(BDIR)/main.o
LINK           ?= $(BOOTDIR)/linker.ld $(OBJS) $(BDIR)/initram.elf.embed
LIBS           ?=



include src/Makefile


all: $(BDIR)/kernel.bin
	@wc -c $^

# running
run: all
	$(QEMU) -M versatilepb -m 32M -nographic -kernel $(BDIR)/kernel.bin

# kernel (binary image)
$(BDIR)/kernel.bin: $(BDIR)/kernel.elf
	$(OBJCOPY) -O binary $^ $@





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
