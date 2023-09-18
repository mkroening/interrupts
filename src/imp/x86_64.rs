use core::arch::asm;

pub type Flags = bool;

#[inline]
pub fn read_disable() -> Flags {
    let rflags: u64;

    unsafe {
        asm!(
            "pushfq",
            "pop {}",
            "cli",
            out(reg) rflags,
            options(nomem, preserves_flags)
        );
    }

    const INTERRUPT_FLAG: u64 = 1 << 9;

    (rflags & INTERRUPT_FLAG) == INTERRUPT_FLAG
}

#[inline]
pub fn restore(enable: Flags) {
    if enable {
        unsafe {
            asm!("sti", options(nomem, preserves_flags));
        }
    }
}
