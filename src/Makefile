
### Tools and build directories

RUSTC          ?= $(RUST_ROOT)/bin/rustc
CARGO          ?= $(CARGO_ROOT)/bin/cargo-build
CARGOCLEAN     ?= $(CARGO_ROOT)/bin/cargo-clean
# CC is probably defined (as GCC)
CC              = $(LLVM_ROOT)/bin/clang
LLC            ?= $(LLVM_ROOT)/bin/llc
NASM           ?= nasm

# Cross-compiled binutils
AR              = $(GCC_PREFIX)ar
AS              = $(GCC_PREFIX)as
LD              = $(GCC_PREFIX)ld
GDB             = $(GCC_PREFIX)gdbtui
OBJCOPY         = $(GCC_PREFIX)objcopy

# Flags
CFLAGS         ?= -target $(TARGET) $(MAYBE_DEBUG) -ffreestanding \
				-fdata-sections -ffunction-sections $(MAYBE_CLANG_OPTIMIZE)
RUSTCFLAGS     ?= --target $(TARGET) --emit=obj $(MAYBE_RUSTC_OPTIMIZE) -Z no-landing-pads
				# -C llvm-args="-fdata-sections -ffunction-sections -float-abi=hard"
GCC_PREFIX      = $(GCC_ROOT)/$(GCC_TARGET)/bin/

# Sources and destinations
BDIR            = build/$(arch)
TARGETDIR       = ./target/$(TARGET)/release
ARCHDIR			= ./src/arch/$(arch)
BOOTDIR         = $(ARCHDIR)/boot

# Semifinal targets
OBJS           ?= $(BDIR)/loader.o $(BDIR)/runtime.o $(BDIR)/morestack.o $(BDIR)/main.o
LINK           ?= $(BOOTDIR)/linker.ld $(OBJS) $(BDIR)/initram.elf.embed
LIBS           ?=

