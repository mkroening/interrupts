cfg_select! {
    all(unix, not(miri)) => {
        mod unix;
        pub use self::unix::*;
    }
    all(target_os = "none", target_arch = "aarch64") => {
        mod aarch64;
        pub use self::aarch64::*;
    }
    all(target_os = "none", target_arch = "riscv64") => {
        mod riscv64;
        pub use self::riscv64::*;
    }
    all(target_os = "none", target_arch = "x86_64") => {
        mod x86_64;
        pub use self::x86_64::*;
    }
    _ => {
        mod unsupported;
        pub use self::unsupported::*;
    }
}
