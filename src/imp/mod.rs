cfg_if::cfg_if! {
    if #[cfg(all(unix, not(miri)))] {
        mod unix;
        pub use self::unix::*;
    } else if #[cfg(all(target_os = "none", target_arch = "aarch64"))] {
        mod aarch64;
        pub use self::aarch64::*;
    } else if #[cfg(all(target_os = "none", target_arch = "riscv64"))] {
        mod riscv64;
        pub use self::riscv64::*;
    } else if #[cfg(all(target_os = "none", target_arch = "x86_64"))] {
        mod x86_64;
        pub use self::x86_64::*;
    } else {
        mod unsupported;
        pub use self::unsupported::*;
    }
}
