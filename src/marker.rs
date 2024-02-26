/// Types that can be transferred across interrupt handler boundaries on the same hardware thread.
///
/// Exactly like [`Send`] but for interrupt handlers.
pub unsafe trait InterruptSend {}

unsafe impl<T: ?Sized + Send> InterruptSend for T {}

/// Types for which it is safe to share references with interrupt handlers on the same hardware thread.
///
/// Exactly like [`Sync`] but for interrupt handlers.
pub unsafe trait InterruptSync {}

unsafe impl<T: ?Sized + Sync> InterruptSync for T {}
