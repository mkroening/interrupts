use core::arch::asm;

pub type Flags = u64;

#[inline]
pub fn read_disable() -> Flags {
    let daif: Flags;
    unsafe {
        asm!(
            "mrs {}, DAIF",
            "msr DAIFSet, 0b111",
            out(reg) daif,
            // Omit `nomem` to imitate a lock acquire.
            // Otherwise, the compiler is free to move
            // reads and writes through this asm block.
            options(nostack)
        );
    }
    daif
}

#[inline]
pub fn restore(daif: Flags) {
    unsafe {
        asm!(
            "msr DAIF, {}",
            in(reg) daif,
            // Omit `nomem` to imitate a lock release.
            // Otherwise, the compiler is free to move
            // reads and writes through this asm block.
            options(nostack)
        );
    }
}
