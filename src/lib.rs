use {
    core::ops::{AddAssign, DivAssign, MulAssign, Rem, RemAssign, SubAssign},
    num_traits::{FromPrimitive, PrimInt, ToPrimitive, WrappingAdd, WrappingMul, WrappingSub},
    std::num::{NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize},
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
pub trait NonZero<U: Unsigned>: Copy {
    fn new(val: U) -> Option<Self>;

    unsafe fn new_unchecked(val: U) -> Self;

    fn get(self) -> U;
}

impl NonZero<u8> for NonZeroU8 {
    fn new(val: u8) -> Option<Self> {
        Self::new(val)
    }

    unsafe fn new_unchecked(val: u8) -> Self {
        Self::new_unchecked(val)
    }

    fn get(self) -> u8 {
        self.get()
    }
}

impl NonZero<u16> for NonZeroU16 {
    fn new(val: u16) -> Option<Self> {
        Self::new(val)
    }

    unsafe fn new_unchecked(val: u16) -> Self {
        Self::new_unchecked(val)
    }

    fn get(self) -> u16 {
        self.get()
    }
}

impl NonZero<u32> for NonZeroU32 {
    fn new(val: u32) -> Option<Self> {
        Self::new(val)
    }

    unsafe fn new_unchecked(val: u32) -> Self {
        Self::new_unchecked(val)
    }

    fn get(self) -> u32 {
        self.get()
    }
}

impl NonZero<u64> for NonZeroU64 {
    fn new(val: u64) -> Option<Self> {
        Self::new(val)
    }

    unsafe fn new_unchecked(val: u64) -> Self {
        Self::new_unchecked(val)
    }

    fn get(self) -> u64 {
        self.get()
    }
}

impl NonZero<usize> for NonZeroUsize {
    fn new(val: usize) -> Option<Self> {
        Self::new(val)
    }

    unsafe fn new_unchecked(val: usize) -> Self {
        Self::new_unchecked(val)
    }

    fn get(self) -> usize {
        self.get()
    }
}
