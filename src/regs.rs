use core::arch::asm;

pub(crate) fn read_sp() -> usize {
    let sp: usize;

    unsafe {
        asm!(
            "mv {}, sp",
            out(reg) sp,
            options(nomem, nostack, preserves_flags)
        );
    }
    sp
}
