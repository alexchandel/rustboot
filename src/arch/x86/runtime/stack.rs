pub static RED_ZONE: uint = 5 * 4 * 1024;

#[cfg(target_arch = "arm")] #[inline(always)]
fn get_tls() -> uint {
    let tls_addr;
    asm!(
        "mrc p15, #0, $0, c13, c0, #3

        cmp $0, #0
        mvneq $0, #0xF000
        ldreq $0, [r3, #-15]

        add $0, $0, #4"
        : "=r"(tls_addr));
    return tls_addr;
}

#[inline(always)]
pub unsafe fn record_sp_limit(limit: uint) {
    return target_record_sp_limit(limit);

    // x86-64
    #[cfg(target_arch = "x86_64")] #[inline(always)]
    unsafe fn target_record_sp_limit(limit: uint) {
        asm!("movq $0, %fs:112" :: "r"(limit) :: "volatile")
    }

    // x86
    #[cfg(target_arch = "x86")] #[inline(always)]
    unsafe fn target_record_sp_limit(limit: uint) {
        asm!("movl $0, %gs:48" :: "r"(limit) :: "volatile")
    }

    #[cfg(target_arch = "arm")] #[inline(always)]
    unsafe fn target_record_sp_limit(limit: uint) {
        asm!(
            "str $0, [$1]
            mov pc, lr"
            :: "r"(limit), "r"(tls_addr()) :: "volatile")
    }
}

#[inline(always)]
pub unsafe fn get_sp_limit() -> uint {
    return target_get_sp_limit();

    // x86-64
    #[cfg(target_arch = "x86_64")] #[inline(always)]
    unsafe fn target_get_sp_limit() -> uint {
        let limit;
        asm!("movq %fs:112, $0" : "=r"(limit));
        return limit;
    }

    // x86
    #[cfg(target_arch = "x86")] #[inline(always)]
    unsafe fn target_get_sp_limit() -> uint {
        let limit;
        asm!("movl %gs:48, $0" : "=r"(limit));
        return limit;
    }

    #[cfg(target_arch = "arm")] #[inline(always)]
    unsafe fn target_get_sp_limit() -> uint {
        let limit;
        asm!(
            "ldr $0, [$1]
            mov pc, lr"
            : "=r"(limit) : "r"(tls_addr()) :: "volatile")
    }
}
