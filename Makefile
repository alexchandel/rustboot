-include ./config.mk

arch           ?= x86

# can make the compiler fail. Disable for now
# DEBUG          ?=

# Opt level set to 2 should be enough.
ifdef DEBUG
MAYBE_DEBUG    ?= -g
else
MAYBE_RUSTC_OPTIMIZE ?= --opt-level=3 -Z lto
MAYBE_CLANG_OPTIMIZE ?= -O2
endif

CARGO_ROOT     ?= /usr
RUST_ROOT      ?= /usr
LLVM_ROOT      ?= /usr
GCC_ROOT       ?= /usr
SHELL          ?= /bin/bash

.PHONY: all run debug clean

include src/arch/$(arch)/Makefile

# Compile rustboot. Depends on ./src/lib.rs $(TARGETDIR)/deps/libcore.rlib $(TARGETDIR)/deps/librlibc.rlib
$(BDIR)/main.o:
	$(CARGO) $(RUSTCFLAGS) $(RUSTCFLAGS_2) --verbose --release
	$(RUSTC) src/lib.rs --crate-name main $(RUSTCFLAGS) $(RUSTCFLAGS_2) --out-dir $(BDIR) -L $(TARGETDIR)/deps
	# $(AR) x $(TARGETDIR)/libmain.a main.o
	# mv main.o $(BDIR)

# assemble main.o or main.s from LLVM main.bc
$(BDIR)/%.o: $(BDIR)/%.bc
	$(CC) $(CFLAGS) -c $^ -o $@
$(BDIR)/%.s: $(BDIR)/%.bc
	$(LLC) $(LLCFLAGS) $^ -o $@

### Assemble loader & initram
$(BDIR)/%.o: $(BOOTDIR)/%.s
	$(AS) $(ASMFLAGS) -o $@ $<
$(BDIR)/%.o: $(BOOTDIR)/%.asm
	$(NASM) $(ASMFLAGS) -o $@ $<

$(BDIR)/initram.o: initram/initram.rs
	$(RUSTC) $(RUSTCFLAGS) $(RUSTCFLAGS_2)  --out-dir $(BDIR) $^
$(BDIR)/initram.elf: $(BDIR)/initram.o
	$(LD) $(LDFLAGS) -s $(BDIR)/initram.o -o $(BDIR)/initram.elf
$(BDIR)/%.embed: $(BDIR)/%
	cd $(@D); $(LD) $(LDFLAGS_EMBED) -r -b binary -o $(@F) $(<F)

# kernel (object)
$(BDIR)/kernel.elf: $(LINK)
	$(LD) $(LDFLAGS) -o $@ -T $^ "-(" $(LIBS) "-)" -Map=$(BDIR)/linker.map

clean:
	$(CARGOCLEAN)
	@cat $(BDIR)/.gitignore | xargs -I{} find $(BDIR) -name {} | xargs rm -f
