use {
    core::ops::{AddAssign, DivAssign, MulAssign, Rem, RemAssign, SubAssign},
    std::num::{NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize},
};

// Re-exporting for convenience.
pub use num_traits::{
    Bounded, FromPrimitive, One, PrimInt, ToPrimitive, WrappingAdd, WrappingMul, WrappingSub,
};

/// An unsigned integer.
/// `u8`, `u16`, `u32`, `u64`, `usize`.
pub trait Unsigned:
    PrimInt
    + num_traits::Unsigned
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
    + WrappingAdd
    + WrappingSub
    + WrappingMul
    + Rem
    + RemAssign
    + FromPrimitive
    + ToPrimitive
{
    const BITS: u32;

    /// [`FromPrimitive::from_u8`] succeeds for all supported types.
    fn from_u8(byte: u8) -> Self;

    /// [`ToPrimitive::to_usize`] succeeds for all supported types,
    /// except for `u32` and `u64` on 16-bit platforms and
    /// `u64` on 32-bit platforms.
    fn to_usize(self) -> usize;
}

impl Unsigned for u8 {
    const BITS: u32 = Self::BITS;

    fn from_u8(byte: u8) -> Self {
        byte as _
    }

    fn to_usize(self) -> usize {
        self as _
    }
}

impl Unsigned for u16 {
    const BITS: u32 = Self::BITS;

    fn from_u8(byte: u8) -> Self {
        byte as _
    }

    fn to_usize(self) -> usize {
        self as _
    }
}

impl Unsigned for u32 {
    const BITS: u32 = Self::BITS;

    fn from_u8(byte: u8) -> Self {
        byte as _
    }

    fn to_usize(self) -> usize {
        // May fail on a 16-bit platform.
        // Don't really care about those, but still leaving the `expect()` just in case.
        <Self as ToPrimitive>::to_usize(&self)
            .expect("tried to convert `u32` to `usize` with overflow")
    }
}

impl Unsigned for u64 {
    const BITS: u32 = Self::BITS;

    fn from_u8(byte: u8) -> Self {
        byte as _
    }

    fn to_usize(self) -> usize {
        // May fail on a 32-bit platform.
        // Don't really care about those, but still leaving the `expect()` just in case.
        <Self as ToPrimitive>::to_usize(&self)
            .expect("tried to convert `u64` to `usize` with overflow")
    }
}

impl Unsigned for usize {
    const BITS: u32 = Self::BITS;

    fn from_u8(byte: u8) -> Self {
        byte as _
    }

    fn to_usize(self) -> usize {
        self
    }
}

/// A non-zero unsigned integer.
/// `NonZeroU8`, `NonZeroU16`, `NonZeroU32`, `NonZeroU64`, `NonZeroUsize`.
pub trait NonZero: Copy {
    type U: Unsigned;

    fn new(val: Self::U) -> Option<Self>;

    unsafe fn new_unchecked(val: Self::U) -> Self;

    fn get(self) -> Self::U;
}

impl NonZero for NonZeroU8 {
    type U = u8;

    fn new(val: Self::U) -> Option<Self> {
        Self::new(val)
    }

    unsafe fn new_unchecked(val: Self::U) -> Self {
        Self::new_unchecked(val)
    }

    fn get(self) -> Self::U {
        self.get()
    }
}

impl NonZero for NonZeroU16 {
    type U = u16;

    fn new(val: Self::U) -> Option<Self> {
        Self::new(val)
    }

    unsafe fn new_unchecked(val: Self::U) -> Self {
        Self::new_unchecked(val)
    }

    fn get(self) -> Self::U {
        self.get()
    }
}

impl NonZero for NonZeroU32 {
    type U = u32;

    fn new(val: Self::U) -> Option<Self> {
        Self::new(val)
    }

    unsafe fn new_unchecked(val: Self::U) -> Self {
        Self::new_unchecked(val)
    }

    fn get(self) -> Self::U {
        self.get()
    }
}

impl NonZero for NonZeroU64 {
    type U = u64;

    fn new(val: Self::U) -> Option<Self> {
        Self::new(val)
    }

    unsafe fn new_unchecked(val: Self::U) -> Self {
        Self::new_unchecked(val)
    }

    fn get(self) -> Self::U {
        self.get()
    }
}

impl NonZero for NonZeroUsize {
    type U = usize;

    fn new(val: Self::U) -> Option<Self> {
        Self::new(val)
    }

    unsafe fn new_unchecked(val: Self::U) -> Self {
        Self::new_unchecked(val)
    }

    fn get(self) -> Self::U {
        self.get()
    }
}
