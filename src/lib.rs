//! Cross-architecture utilities for temporarily disabling interrupts and signals.
//!
//! This crate allows you to temporarily disable interrupts and then restore the previous state again.
//!
//! Supported platforms:
//!
//! -   bare-metal (kernel mode, `target_os = "none"`)
//!
//!     Disables hardware interrupts.
//!
//!     - AArch64 (`arch = aarch64`)
//!
//!     - 64-bit RISC-V (`arch = riscv64`)
//!
//!     - x86-64 (`arch = x86_64`)
//!
//! -   Unix (user mode, `unix`)
//!
//!     Disables signals.
//!
//! On targets with non-unix operating systems (not `cfg!(unix)`), this crate does nothing.
//!
//! # Caveats
//!
//! <div class="warning">Interrupts are disabled on a best-effort basis.</div>
//!
//! Even though this crate makes sure that interrupts are disabled, nothing prevents you from manually enabling them again.
//!
//! [Manually dropping] [`Guard`]s  may also cause interrupts to be enabled.
//!
//! [Manually dropping]: Guard#caveats-drop-order
//!
//! # Examples
//!
//! Use [`disable`] to disable interrupts with a guard:
//!
//! ```
//! // interrupts may or may not be enabled
//! let guard = interrupts::disable();
//! // interrupts are disabled
//! drop(guard);
//! // interrupts are restored to the previous state
//! ```
//!
//! Use [`without`] to run a closure with disabled interrupts:
//!
//! ```
//! // interrupts may or may not be enabled
//! interrupts::without(|| {
//!     // interrupts are disabled
//! });
//! // interrupts are restored to the previous state
//! ```

#![cfg_attr(target_os = "none", no_std)]

mod imp;

use core::marker::PhantomData;
use core::sync::atomic::{compiler_fence, Ordering};

/// Temporarily disable interrupts.
///
/// Interrupts are enabled once the returned [`Guard`] is dropped.
///
/// # Synchronization
///
/// This synchronizes the current thread _with itself_ via a [`compiler_fence`].
///
/// A compiler fence is sufficient for sharing a `!Sync` type, such as [`RefCell`], with an interrupt handler on the same hardware thread (core).
///
/// [`RefCell`]: core::cell::RefCell
///
/// # Examples
///
/// ```
/// // interrupts may or may not be enabled
/// let guard = interrupts::disable();
/// // interrupts are disabled
/// drop(guard);
/// // interrupts are restored to the previous state
/// ```
#[inline]
pub fn disable() -> Guard {
    compiler_fence(Ordering::Acquire);
    Guard {
        flags: imp::read_disable(),
        _not_send: PhantomData,
    }
}

/// An interrupt guard.
///
/// Created using [`disable`].
///
/// While an instance of this guard is held, interrupts are disabled.
/// When this guard is dropped, interrupts are restored to the state before disabling.
///
/// # Caveats (Drop Order)
///
/// If interrupts are enabled, acquiring a guard will disable them.
/// Dropping this guard will enable interrupts again.
/// Different [`Guard`]s might be dropped in arbitrary order.
///
/// This may result in interrupts being enabled again, even though another [`Guard`] is still held.
/// For this to happen, one must explicitly drop guards in the wrong order, though.
/// As long as guards don't leave their original [drop scope], they are dropped automatically in the correct order.
///
/// [drop scope]: https://doc.rust-lang.org/reference/destructors.html#drop-scopes
///
/// # Examples
///
/// ```
/// // interrupts may or may not be enabled
/// let guard = interrupts::disable();
/// // interrupts are disabled
/// drop(guard);
/// // interrupts are restored to the previous state
/// ```
///
/// Dropping guards in the wrong order (don't do this):
///
/// ```
/// // Interrupts are enabled
/// let a = interrupts::disable();
/// // Interrupts are disabled
/// let b = interrupts::disable();
/// drop(a);
/// // Interrupts are enabled, although we still hold a guard in b
/// drop(b);
/// ```
pub struct Guard {
    flags: imp::Flags,
    /// Interrupts are per hardware thread.
    ///
    /// Making Guard `!Send` avoids disabling interrupts on one hardware thread and restoring on another.
    _not_send: PhantomData<*mut ()>,
}

impl Guard {
    /// ```compile_fail
    /// fn send<T: Send>(_: T) {}
    ///
    /// send(interrupts::disable());
    /// ```
    fn _dummy() {}
}

impl Drop for Guard {
    #[inline]
    fn drop(&mut self) {
        compiler_fence(Ordering::Release);
        #[allow(clippy::unit_arg)]
        imp::restore(self.flags);
    }
}

/// Run a closure with disabled interrupts.
///
/// Run the given closure, disabling interrupts before running it (if they aren't already disabled).
/// Afterward, interrupts are enabled again if they were enabled before.
///
/// If you have other `enable` and `disable` calls _within_ the closure, things may not work as expected.
///
/// Only has an effect if `target_os = "none"`.
///
/// # Synchronization
///
/// This synchronizes the current thread _with itself_ via a [`compiler_fence`].
///
/// A compiler fence is sufficient for sharing a `!Sync` type, such as [`RefCell`], with an interrupt handler on the same hardware thread (core).
///
/// [`RefCell`]: core::cell::RefCell
///
/// # Examples
///
/// ```
/// // interrupts may or may not be enabled
/// interrupts::without(|| {
///     // interrupts are disabled
/// });
/// // interrupts are restored to the previous state
/// ```
///
/// Nesting:
///
/// ```
/// // interrupts may be enabled
/// interrupts::without(|| {
///     // interrupts are disabled
///     interrupts::without(|| {
///         // interrupts are disabled
///     });
///     // interrupts are still disabled
/// });
/// // interrupts are restored
/// ```
// Docs adapted from `x86_64::instructions::interrupts::without_interrupts`.
#[inline]
pub fn without<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    let guard = disable();

    let ret = f();

    drop(guard);

    ret
}
