pub type Flags = ();

#[inline]
pub fn read_disable() -> Flags {}

#[inline]
pub fn restore(_flags: Flags) {}
