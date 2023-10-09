use core::arch::asm;

pub type Flags = u8;

#[inline]
pub fn read_disable() -> Flags {
    let flags: Flags;
    unsafe {
        asm!(
            // Atomic Read and Clear Immediate Bits in CSR
            // `csrx rd, csr, rs1`
            // Set SIE and MIE
            "csrrci {rd}, mstatus, 0b1010",
            rd = out(reg) flags,
            // Omit `nomem` to imitate a lock acquire.
            // Otherwise, the compiler is free to move
            // reads and writes through this asm block.
            options(preserves_flags, nostack)
        );
    }
    flags
}

#[inline]
pub fn restore(flags: Flags) {
    unsafe {
        asm!(
            // Atomic Set Bits in CSR
            "csrs mstatus, {rs1}",
            rs1 = in(reg) flags,
            // Omit `nomem` to imitate a lock release.
            // Otherwise, the compiler is free to move
            // reads and writes through this asm block.
            options(preserves_flags, nostack)
        );
    }
}
