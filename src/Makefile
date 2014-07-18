
### Tools and build directories

CARGO          ?= $(RUST_ROOT)/cargo-build
CARGOCLEAN     ?= $(RUST_ROOT)/cargo-clean
RUSTCFLAGS     ?= --target $(TARGET) --emit=obj $(MAYBE_RUSTC_OPTIMIZE) -Z no-landing-pads
				# -C llvm-args="-fdata-sections -ffunction-sections -float-abi=hard"
# RUSTCFLAGS_2 is defined in architecture-specific Makefile

# CC is probably defined (as GCC)
CC              = $(LLVM_ROOT)/bin/clang
LLC            ?= $(LLVM_ROOT)/bin/llc
NASM           ?= nasm

# Cross-compiled binutils
AR              = $(GCC_PREFIX_2)ar
AS              = $(GCC_PREFIX_2)as
LD              = $(GCC_PREFIX_2)ld
GDB             = $(GCC_PREFIX_2)gdbtui
OBJCOPY         = $(GCC_PREFIX_2)objcopy

BDIR            = build/$(arch)
TARGETDIR       = ./target/$(TARGET)/release
ARCHDIR			= ./src/arch/$(arch)
BOOTDIR         = $(ARCHDIR)/boot

