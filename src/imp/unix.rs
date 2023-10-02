use nix::sys::signal::{SigSet, SigmaskHow};

pub type Flags = SigSet;

#[inline]
pub fn read_disable() -> Flags {
    SigSet::all()
        .thread_swap_mask(SigmaskHow::SIG_SETMASK)
        .unwrap()
}

#[inline]
pub fn restore(flags: Flags) {
    flags.thread_set_mask().unwrap();
}

#[cfg(test)]
mod tests {
    #[test]
    fn signals() {
        use core::sync::atomic::{AtomicBool, Ordering};

        use nix::libc;
        use nix::sys::signal::{self, SigHandler, Signal};

        static HANDLER_RAN: AtomicBool = AtomicBool::new(false);

        extern "C" fn handle_sigint(_signal: libc::c_int) {
            HANDLER_RAN.store(true, Ordering::Relaxed);
        }

        let handler = SigHandler::Handler(handle_sigint);
        unsafe { signal::signal(Signal::SIGINT, handler) }.unwrap();

        let guard = crate::disable();
        signal::raise(Signal::SIGINT).unwrap();
        assert!(!HANDLER_RAN.load(Ordering::Relaxed));
        drop(guard);
        assert!(HANDLER_RAN.load(Ordering::Relaxed));
    }
}
