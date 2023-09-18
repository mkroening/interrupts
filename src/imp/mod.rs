#[cfg(not(target_os = "none"))]
mod unsupported;
#[cfg(not(target_os = "none"))]
pub use unsupported::*;

#[cfg(all(target_os = "none", target_arch = "aarch64"))]
mod aarch64;
#[cfg(all(target_os = "none", target_arch = "aarch64"))]
pub use aarch64::*;

#[cfg(all(target_os = "none", target_arch = "riscv64"))]
mod riscv64;
#[cfg(all(target_os = "none", target_arch = "riscv64"))]
pub use riscv64::*;

#[cfg(all(target_os = "none", target_arch = "x86_64"))]
mod x86_64;
#[cfg(all(target_os = "none", target_arch = "x86_64"))]
pub use x86_64::*;
