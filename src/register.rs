use core::{cmp, convert, marker, ops};

/// A value that a register can store.
///
/// All registers are either `u8` or `u16`.
pub trait RegisterValue : Copy + Clone +
               ops::BitAnd<Output=Self> +
               ops::BitAndAssign +
               ops::BitOr<Output=Self> +
               ops::BitOrAssign +
               ops::BitXor<Output=Self> +
               ops::BitXorAssign +
               ops::Not<Output=Self> +
               cmp::PartialEq + cmp::Eq +
               cmp::PartialOrd + cmp::Ord +
               convert::From<u8> {
}

/// A register.
pub trait Register : Sized {
    /// The type that can represent the value of the register.
    type T: RegisterValue;
    /// The type representing a set of bits that may be manipulated
    /// within the register.
    type RegisterBits = RegisterBits<Self>;

    /// The address of the register.
    const ADDRESS: *mut Self::T;

    /// Writes a value to the register.
    #[inline(always)]
    fn write<V>(value: V) where V: Into<Self::T> {
        unsafe {
            *Self::ADDRESS = value.into();
        }
    }

    /// Reads the value of the register.
    #[inline(always)]
    fn read() -> Self::T {
        unsafe { *Self::ADDRESS }
    }

    /// Sets a set of bits to `1` in the register.
    fn set(bits: RegisterBits<Self>) {
        Self::set_mask_raw(bits.mask);
    }

    /// Sets a bitmask in a register.
    ///
    /// This is equivalent to `r |= mask`.
    #[inline(always)]
    fn set_mask_raw(mask: Self::T) {
        unsafe {
            *Self::ADDRESS |= mask;
        }
    }

    /// Unsets a set of bits in the register.
    ///
    /// All of the bits will be set to `0`.
    fn unset(bits: RegisterBits<Self>) {
        Self::unset_mask_raw(bits.mask);
    }

    /// Clears a bitmask from a register.
    ///
    /// This is equivalent to `r &= !mask`.
    #[inline(always)]
    fn unset_mask_raw(mask: Self::T) {
        unsafe {
            *Self::ADDRESS &= !mask;
        }
    }

    /// Toggles a set of bits within the register.
    ///
    /// All specified bits which were previously `0` will become
    /// `1`, and all specified bits that were previous `1` will
    /// become `0`.
    fn toggle(mask: RegisterBits<Self>) {
        Self::toggle_raw(mask.mask);
    }

    /// Toggles a mask in the register.
    ///
    /// This is equivalent to `r ^= mask`.
    #[inline(always)]
    fn toggle_raw(mask: Self::T) {
        unsafe {
            *Self::ADDRESS ^= mask;
        }
    }

    /// Checks if a set of bits are enabled.
    ///
    /// All specifed bits must be set for this function
    /// to return `true`.
    fn is_set(bits: RegisterBits<Self>) -> bool {
        Self::is_mask_set_raw(bits.mask)
    }

    /// Checks if a mask is set in the register.
    ///
    /// This is equivalent to `(r & mask) == mask`.
    #[inline(always)]
    fn is_mask_set_raw(mask: Self::T) -> bool {
        unsafe {
            (*Self::ADDRESS & mask) == mask
        }
    }

    /// Checks if a set of bits are not set.
    ///
    /// All specified bits must be `0` for this
    /// function to return `true`.
    fn is_clear(mask: RegisterBits<Self>) -> bool {
        Self::is_clear_raw(mask.mask)
    }

    /// Checks if a mask is clear in the register.
    ///
    /// This is equivalent to `(r & mask) == 0`.
    #[inline(always)]
    fn is_clear_raw(mask: Self::T) -> bool {
        unsafe {
            (*Self::ADDRESS & mask) == Self::T::from(0)
        }
    }

    /// Waits until a set of bits are set in the register.
    ///
    /// This function will block until all bits that are set in
    /// the mask are also set in the register.
    fn wait_until_set(bits: RegisterBits<Self>) {
        Self::wait_until_mask_set_raw(bits.mask);
    }

    /// Waits until a bit mask is set in the register.
    ///
    /// This function will block until all bits that are set in
    /// the mask are also set in the register.
    #[inline(always)]
    fn wait_until_mask_set_raw(mask: Self::T) {
        wait_until(|| Self::is_mask_set_raw(mask))
    }
}

/// Represents a set of bits within a specific register.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RegisterBits<R: Register> {
    /// The raw bitmask.
    mask: R::T,
    _phantom: marker::PhantomData<R>,
}

impl<R> RegisterBits<R> where R: Register {
    /// Creates a new register mask.
    pub const fn new(mask: R::T) -> Self {
        RegisterBits { mask, _phantom: marker::PhantomData }
    }

    pub fn zero() -> Self {
        RegisterBits::new(0u8.into())
    }
}

impl<R> ops::BitOr for RegisterBits<R> where R: Register
{
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        RegisterBits::new(self.mask | rhs.mask)
    }
}

impl<R> ops::BitOrAssign for RegisterBits<R> where R: Register {
    fn bitor_assign(&mut self, rhs: Self) {
        self.mask |= rhs.mask;
    }
}

impl<R> ops::BitAnd for RegisterBits<R> where R: Register
{
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        RegisterBits::new(self.mask & rhs.mask)
    }
}

impl<R> ops::BitAndAssign for RegisterBits<R> where R: Register {
    fn bitand_assign(&mut self, rhs: Self) {
        self.mask &= rhs.mask;
    }
}

impl<R> ops::Not for RegisterBits<R> where R: Register {
    type Output = Self;

    fn not(self) -> Self {
        RegisterBits::new(!self.mask)
    }
}

impl<R> From<RegisterBits<R>> for u8 where R: Register<T=u8> {
    fn from(other: RegisterBits<R>) -> u8 { other.mask }
}

impl<R> From<RegisterBits<R>> for u16 where R: Register<T=u16> {
    fn from(other: RegisterBits<R>) -> u16 { other.mask }
}

impl RegisterValue for u8 { }
impl RegisterValue for u16 { }

/// Waits until some condition is true of the register.
#[inline(always)]
fn wait_until<F>(mut f: F)
    where F: FnMut() -> bool {
    loop {
        if f() {
            break;
        }
    }
}

