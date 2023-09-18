//! Cross-architecture utilities for temporarily disabling interrupts.
//!
//! This crate allows you to temporarily disable interrupts and then restore the previous state again.
//!
//! Supported architectures:
//! - AArch64 (`aarch64`)
//! - 64-bit RISC-V (`riscv64`)
//! - x86-64 (`x86_64`)
//!
//! When used on targets with an operating system (not `cfg!(target_os = "none")`), this crate does nothing.
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

#![no_std]

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
/// # Examples
///
/// ```
/// // interrupts may or may not be enabled
/// let guard = interrupts::disable();
/// // interrupts are disabled
/// drop(guard);
/// // interrupts are restored to the previous state
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
