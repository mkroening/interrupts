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
            options(nomem, nostack)
        );
    }
    daif
}

#[inline]
pub fn restore(daif: Flags) {
    unsafe {
        asm!("msr DAIF, {}", in(reg) daif, options(nomem, nostack));
    }
}
