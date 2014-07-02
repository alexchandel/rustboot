-include ./config.mk
-include $(BDIR)/main.d

# need to create new variable due to semantics of make "recursive variables"
GCC_PREFIX_2   ?= $(GCC_PREFIX)

# Opt level set to 2 should be enough.
ifdef DEBUG
MAYBE_DEBUG    ?= -g
endif

ifndef DEBUG
MAYBE_RUSTC_OPTIMIZE ?= --opt-level=2 -Z lto
MAYBE_CLANG_OPTIMIZE ?= -O2
endif

RUSTC          ?= $(RUST_ROOT)/bin/rustc
RUSTCFLAGS     ?= --target $(TARGET) -Z no-landing-pads $(MAYBE_RUSTC_OPTIMIZE)

# CC is probably defined (as GCC)
CC              = $(LLVM_ROOT)/bin/clang
CFLAGS         ?= -ffreestanding -target $(TARGET) $(MAYBE_CLANG_OPTIMIZE) $(MAYBE_DEBUG) -fdata-sections -ffunction-sections

LLC            ?= $(LLVM_ROOT)/bin/llc

# AS is probably defined
AS              = $(GCC_PREFIX_2)as
LD              = $(GCC_PREFIX_2)ld
GDB             = $(GCC_PREFIX_2)gdbtui
OBJCOPY         = $(GCC_PREFIX_2)objcopy

BDIR           ?= ./boot

DEP_SCRIPT     ?= 's~\($(DEP_KEEP)\)~\n\1~g;s~ \S*$(DEP_RM)\S*~~g;s~\n\($(DEP_KEEP)\)~\1~g'


# Compile rustboot. Produce dependency info
$(BDIR)/main.o: ../../common/lib.rs
	$(RUSTC) $(RUSTCFLAGS) -L $(BDIR) --out-dir $(BDIR) --dep-info --emit=obj $<
	@sed -e $(DEP_SCRIPT) $(BDIR)/main.d > $(BDIR)/main.d.tmp
	@mv $(BDIR)/main.d.tmp $(BDIR)/main.d		# SED to tmp file for OS X compatibility

# initram

$(BDIR)/initram.elf: $(BDIR)/module.o
	$(LD) $(LDFLAGS) -s $< -o $@

$(BDIR)/%.embed: $(BDIR)/%
	cd $(@D); $(LD) $(LDFLAGS_EMBED) -r -b binary -o $(@F) $(<F)

.PHONY: clean
clean:
	@cat $(BDIR)/.gitignore | xargs -I{} find $(BDIR) -name {} | xargs rm -f
