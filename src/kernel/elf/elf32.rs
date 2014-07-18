use core::mem::transmute;
use core::ptr::copy_nonoverlapping_memory;

use rust_core::c_types::{c_ushort, c_uint, c_int, c_ulong, c_long};

use kernel::process::Process;

// rust-bindgen generated bindings
pub type Elf32_Half = c_ushort;
pub type Elf32_Word = c_uint;
pub type Elf32_Sword = c_int;
pub type Elf32_Xword = c_ulong;
pub type Elf32_Sxword = c_long;
pub type Elf32_Addr = c_uint;
pub type Elf32_Off = c_uint;
pub type Elf32_Section = c_ushort;
pub type Elf32_Symndx = c_uint;
type c_uchar = u8;
type c_void = uint;

#[packed]
pub struct Ehdr {
    pub e_ident: [c_uchar, ..16u],
    pub e_type: Elf32_Half,
    pub e_machine: Elf32_Half,
    pub e_version: Elf32_Word,
    pub e_entry: Elf32_Addr,
    pub e_phoff: Elf32_Off,
    pub e_shoff: Elf32_Off,
    pub e_flags: Elf32_Word,
    pub e_ehsize: Elf32_Half,
    pub e_phentsize: Elf32_Half,
    pub e_phnum: Elf32_Half,
    pub e_shentsize: Elf32_Half,
    pub e_shnum: Elf32_Half,
    pub e_shstrndx: Elf32_Half,
}

#[packed]
pub struct Phdr {
    pub p_type: super::HeaderType,
    pub p_offset: Elf32_Off,
    pub p_vaddr: Elf32_Addr,
    pub p_paddr: Elf32_Addr,
    pub p_filesz: Elf32_Word,
    pub p_memsz: Elf32_Word,
    pub p_flags: super::HeaderFlags,
    pub p_align: Elf32_Word,
}

#[packed]
pub struct Elf32_Shdr {
    sh_name: Elf32_Word,
    sh_type: Elf32_Word,
    sh_flags: Elf32_Word,
    sh_addr: Elf32_Addr,
    sh_offset: Elf32_Off,
    sh_size: Elf32_Word,
    sh_link: Elf32_Word,
    sh_info: Elf32_Word,
    sh_addralign: Elf32_Word,
    sh_entsize: Elf32_Word,
}

pub struct Elf32_Sym {
    st_name: Elf32_Word,
    st_value: Elf32_Addr,
    st_size: Elf32_Word,
    st_info: c_uchar,
    st_other: c_uchar,
    st_shndx: Elf32_Section,
}

pub struct Elf32_Syminfo {
    si_boundto: Elf32_Half,
    si_flags: Elf32_Half,
}

pub struct Elf32_Rel {
    r_offset: Elf32_Addr,
    r_info: Elf32_Word,
}

pub struct Elf32_Rela {
    r_offset: Elf32_Addr,
    r_info: Elf32_Word,
    r_addend: Elf32_Sword,
}

pub struct Union_Unnamed1 {
    data: [c_uchar, ..4u],
}
impl Union_Unnamed1 {
    pub fn d_val(&mut self) -> *mut Elf32_Word {
        unsafe { transmute(self) }
    }
    pub fn d_ptr(&mut self) -> *mut Elf32_Addr {
        unsafe { transmute(self) }
    }
}
pub struct Elf32_Dyn {
    d_tag: Elf32_Sword,
    d_un: Union_Unnamed1,
}

pub struct Elf32_Verdef {
    vd_version: Elf32_Half,
    vd_flags: Elf32_Half,
    vd_ndx: Elf32_Half,
    vd_cnt: Elf32_Half,
    vd_hash: Elf32_Word,
    vd_aux: Elf32_Word,
    vd_next: Elf32_Word,
}

pub struct Elf32_Verdaux {
    vda_name: Elf32_Word,
    vda_next: Elf32_Word,
}

pub struct Elf32_Verneed {
    vn_version: Elf32_Half,
    vn_cnt: Elf32_Half,
    vn_file: Elf32_Word,
    vn_aux: Elf32_Word,
    vn_next: Elf32_Word,
}

pub struct Elf32_Vernaux {
    vna_hash: Elf32_Word,
    vna_flags: Elf32_Half,
    vna_other: Elf32_Half,
    vna_name: Elf32_Word,
    vna_next: Elf32_Word,
}

pub struct AuxvValue {
    pub data: c_int, // or 8u?
}

impl AuxvValue {
    pub fn a_val(&mut self) -> *mut c_int {
        unsafe { transmute(self) }
    }
    pub fn a_ptr(&mut self) -> *mut *mut c_void {
        // WARN: cannot use 32 bit pointers on x86_64!
        unsafe { transmute(self) }
    }
    pub fn a_fcn(&mut self) -> *mut extern fn() {
        unsafe { transmute(self) }
    }
}

pub struct Auxv {
    pub a_type: AuxvType,
    pub a_un: AuxvValue,
}

/* Legal values for a_type (entry type).  */
#[repr(u32)]
pub enum AuxvType {
    AT_NULL     = 0,       /* End of vector */
    AT_IGNORE   = 1,       /* Entry should be ignored */
    AT_EXECFD   = 2,       /* File descriptor of program */
    AT_PHDR     = 3,       /* Program headers for program */
    AT_PHENT    = 4,       /* Size of program header entry */
    AT_PHNUM    = 5,       /* Number of program headers */
    AT_PAGESZ   = 6,       /* System page size */
    AT_BASE     = 7,       /* Base address of interpreter */
    AT_FLAGS    = 8,       /* Flags */
    AT_ENTRY    = 9,       /* Entry point of program */
    AT_NOTELF   = 10,      /* Program is not ELF */
    AT_UID      = 11,      /* Real uid */
    AT_EUID     = 12,      /* Effective uid */
    AT_GID      = 13,      /* Real gid */
    AT_EGID     = 14,      /* Effective gid */
    AT_CLKTCK   = 17,      /* Frequency of times() */

/* Some more special a_type values describing the hardware.  */
    AT_PLATFORM = 15,      /* String identifying platform.  */
    AT_HWCAP    = 16,      /* Machine-dependent hints about
                       processor capabilities.  */

/* This entry gives some information about the FPU initialization
   performed by the kernel.  */
    AT_FPUCW    = 18,      /* Used FPU control word.  */

/* Cache block sizes.  */
    AT_DCACHEBSIZE = 19,      /* Data cache block size.  */
    AT_ICACHEBSIZE = 20,      /* Instruction cache block size.  */
    AT_UCACHEBSIZE = 21,      /* Unified cache block size.  */

/* A special ignored value for PPC, used by the kernel to control the
   interpretation of the AUXV. Must be > 16.  */
    AT_IGNOREPPC   = 22,      /* Entry should be ignored.  */

    AT_SECURE = 23,      /* Boolean, was exec setuid-like?  */

    AT_BASE_PLATFORM = 24,     /* String identifying real platforms.*/

    AT_RANDOM = 25,      /* Address of 16 random bytes.  */

    AT_HWCAP2 = 26,      /* More machine-dependent hints about
                       processor capabilities.  */

    AT_EXECFN = 31,      /* Filename of executable.  */

/* Pointer to the global system page used for system calls and other
   nice things.  */
    AT_SYSINFO = 32,
    AT_SYSINFO_EHDR = 33,

/* Shapes of the caches.  Bits 0-3 contains associativity; bits 4-7 contains
   log2 of line size; mask those to get cache size.  */
    AT_L1I_CACHESHAPE = 34,
    AT_L1D_CACHESHAPE = 35,
    AT_L2_CACHESHAPE  = 36,
    AT_L3_CACHESHAPE  = 37
}

pub struct Elf32_Nhdr {
    n_namesz: Elf32_Word,
    n_descsz: Elf32_Word,
    n_type: Elf32_Word,
}

pub struct Struct_Unnamed5 {
    gt_current_g_value: Elf32_Word,
    gt_unused: Elf32_Word,
}
pub struct Struct_Unnamed6 {
    gt_g_value: Elf32_Word,
    gt_bytes: Elf32_Word,
}
pub struct Elf32_gptab {
    data: [c_uchar, ..8u],
}
impl Elf32_gptab {
    pub fn gt_header(&mut self) -> *mut Struct_Unnamed5 {
        unsafe { transmute(self) }
    }
    pub fn gt_entry(&mut self) -> *mut Struct_Unnamed6 {
        unsafe { transmute(self) }
    }
}
pub struct Elf32_RegInfo {
    ri_gprmask: Elf32_Word,
    ri_cprmask: [Elf32_Word, ..4u],
    ri_gp_value: Elf32_Sword,
}
pub struct Elf_Options {
    kind: c_uchar,
    size: c_uchar,
    section: Elf32_Section,
    info: Elf32_Word,
}
pub struct Elf_Options_Hw {
    hwp_flags1: Elf32_Word,
    hwp_flags2: Elf32_Word,
}
pub struct Elf32_Lib {
    l_name: Elf32_Word,
    l_time_stamp: Elf32_Word,
    l_checksum: Elf32_Word,
    l_version: Elf32_Word,
    l_flags: Elf32_Word,
}

pub type Elf32_Conflict = Elf32_Addr;
