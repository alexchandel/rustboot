use core::mem::transmute;
use core::intrinsics::offset;
use core::c_types::{c_ushort, c_uint, c_int, c_ulong, c_long};

use util::int::range;

// rust-bindgen generated bindings
pub type Elf64_Half = c_ushort;
pub type Elf64_Word = c_uint;
pub type Elf64_Sword = c_int;
pub type Elf64_Xword = c_ulong;
pub type Elf64_Sxword = c_long;
pub type Elf64_Addr = c_ulong;
pub type Elf64_Off = c_ulong;
pub type Elf64_Section = c_ushort;
pub type Elf64_Symndx = c_ulong;
type c_uchar = u8;
type c_void = uint;

#[packed]
pub struct Ehdr {
    e_ident: [c_uchar, ..16u],
    e_type: Elf64_Half,
    e_machine: Elf64_Half,
    e_version: Elf64_Word,
    e_entry: Elf64_Addr,
    e_phoff: Elf64_Off,
    e_shoff: Elf64_Off,
    e_flags: Elf64_Word,
    e_ehsize: Elf64_Half,
    e_phentsize: Elf64_Half,
    e_phnum: Elf64_Half,
    e_shentsize: Elf64_Half,
    e_shnum: Elf64_Half,
    e_shstrndx: Elf64_Half,
}

#[packed]
pub struct Elf64_Phdr {
    pub p_type: super::HeaderType,
    pub p_flags: Elf64_Word,
    pub p_offset: Elf64_Off,
    pub p_vaddr: Elf64_Addr,
    pub p_paddr: Elf64_Addr,
    pub p_filesz: Elf64_Xword,
    pub p_memsz: Elf64_Xword,
    pub p_align: Elf64_Xword,
}

#[packed]
pub struct Elf64_Shdr {
    sh_name: Elf64_Word,
    sh_type: Elf64_Word,
    sh_flags: Elf64_Xword,
    sh_addr: Elf64_Addr,
    sh_offset: Elf64_Off,
    sh_size: Elf64_Xword,
    sh_link: Elf64_Word,
    sh_info: Elf64_Word,
    sh_addralign: Elf64_Xword,
    sh_entsize: Elf64_Xword,
}

pub struct Elf64_Sym {
    st_name: Elf64_Word,
    st_info: c_uchar,
    st_other: c_uchar,
    st_shndx: Elf64_Section,
    st_value: Elf64_Addr,
    st_size: Elf64_Xword,
}
pub struct Elf64_Syminfo {
    si_boundto: Elf64_Half,
    si_flags: Elf64_Half,
}
pub struct Elf64_Rel {
    r_offset: Elf64_Addr,
    r_info: Elf64_Xword,
}
pub struct Elf64_Rela {
    r_offset: Elf64_Addr,
    r_info: Elf64_Xword,
    r_addend: Elf64_Sxword,
}

pub struct Union_Unnamed2 {
    data: [c_uchar, ..8u],
}
impl Union_Unnamed2 {
    pub fn d_val(&mut self) -> *mut Elf64_Xword {
        unsafe { transmute(self) }
    }
    pub fn d_ptr(&mut self) -> *mut Elf64_Addr {
        unsafe { transmute(self) }
    }
}
pub struct Elf64_Dyn {
    d_tag: Elf64_Sxword,
    d_un: Union_Unnamed2,
}
pub struct Elf64_Verdef {
    vd_version: Elf64_Half,
    vd_flags: Elf64_Half,
    vd_ndx: Elf64_Half,
    vd_cnt: Elf64_Half,
    vd_hash: Elf64_Word,
    vd_aux: Elf64_Word,
    vd_next: Elf64_Word,
}
pub struct Elf64_Verdaux {
    vda_name: Elf64_Word,
    vda_next: Elf64_Word,
}
pub struct Elf64_Verneed {
    vn_version: Elf64_Half,
    vn_cnt: Elf64_Half,
    vn_file: Elf64_Word,
    vn_aux: Elf64_Word,
    vn_next: Elf64_Word,
}
pub struct Elf64_Vernaux {
    vna_hash: Elf64_Word,
    vna_flags: Elf64_Half,
    vna_other: Elf64_Half,
    vna_name: Elf64_Word,
    vna_next: Elf64_Word,
}

pub struct AuxvValue {
    data: [c_uchar, ..8u],
}

impl AuxvValue {
    pub fn a_val(&mut self) -> *mut c_long {
        unsafe { transmute(self) }
    }
    pub fn a_ptr(&mut self) -> *mut *mut c_void {
        unsafe { transmute(self) }
    }
    pub fn a_fcn(&mut self) -> *mut extern fn() {
        unsafe { transmute(self) }
    }
}

pub struct Auxv {
    a_type: c_long,
    a_un: AuxvValue,
}

pub struct Elf64_Nhdr {
    n_namesz: Elf64_Word,
    n_descsz: Elf64_Word,
    n_type: Elf64_Word,
}
pub struct Elf64_Lib {
    l_name: Elf64_Word,
    l_time_stamp: Elf64_Word,
    l_checksum: Elf64_Word,
    l_version: Elf64_Word,
    l_flags: Elf64_Word,
}
