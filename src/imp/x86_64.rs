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
            // Omit `nomem` to imitate a lock acquire.
            // Otherwise, the compiler is free to move
            // reads and writes through this asm block.
            options(preserves_flags)
        );
    }

    const INTERRUPT_FLAG: u64 = 1 << 9;

    (rflags & INTERRUPT_FLAG) == INTERRUPT_FLAG
}

#[inline]
pub fn restore(enable: Flags) {
    if enable {
        unsafe {
            asm!(
                "sti",
                // Omit `nomem` to imitate a lock acquire.
                // Otherwise, the compiler is free to move
                // reads and writes through this asm block.
                options(preserves_flags)
            );
        }
    }
}
