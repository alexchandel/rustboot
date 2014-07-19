
### Tools and build directories

RUSTC          ?= $(RUST_ROOT)/bin/rustc
CARGO          ?= $(CARGO_ROOT)/bin/cargo-build
CARGOCLEAN     ?= $(CARGO_ROOT)/bin/cargo-clean
RUSTCFLAGS     ?= --target $(TARGET) --emit=obj $(MAYBE_RUSTC_OPTIMIZE) -Z no-landing-pads
				# -C llvm-args="-fdata-sections -ffunction-sections -float-abi=hard"
# RUSTCFLAGS_2 is defined in architecture-specific Makefile

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

BDIR            = build/$(arch)
TARGETDIR       = ./target/$(TARGET)/release
ARCHDIR			= ./src/arch/$(arch)
BOOTDIR         = $(ARCHDIR)/boot

