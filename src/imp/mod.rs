cfg_select! {
    target_arch = "aarch64" => {
        mod aarch64;
        pub use self::aarch64::*;
    }
    target_arch = "riscv64" => {
        mod riscv64;
        pub use self::riscv64::*;
    }
    target_arch = "x86_64" => {
        mod x86_64;
        pub use self::x86_64::*;
    }
    _ => {
        mod unsupported;
        pub use self::unsupported::*;
    }
}
