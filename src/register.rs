use core::{cmp, convert, marker, ops};

/// A value that a register can store.
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
    type T: RegisterValue;
    type Mask = Mask<Self>;

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

    fn set(mask: Mask<Self>) {
        Self::set_raw(mask.mask);
    }

    /// Sets a bitmask in a register.
    ///
    /// This is equivalent to `r |= mask`.
    #[inline(always)]
    fn set_raw(mask: Self::T) {
        unsafe {
            *Self::ADDRESS |= mask;
        }
    }

    fn unset(mask: Mask<Self>) {
        Self::unset_raw(mask.mask);
    }

    /// Clears a bitmask from a register.
    ///
    /// This is equivalent to `r &= !mask`.
    #[inline(always)]
    fn unset_raw(mask: Self::T) {
        unsafe {
            *Self::ADDRESS &= !mask;
        }
    }

    fn toggle(mask: Mask<Self>) {
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

    fn is_set(mask: Mask<Self>) -> bool {
        Self::is_set_raw(mask.mask)
    }

    /// Checks if a mask is set in the register.
    ///
    /// This is equivalent to `(r & mask) == mask`.
    #[inline(always)]
    fn is_set_raw(mask: Self::T) -> bool {
        unsafe {
            (*Self::ADDRESS & mask) == mask
        }
    }

    fn is_clear(mask: Mask<Self>) -> bool {
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

    fn wait_until_set(mask: Mask<Self>) {
        Self::wait_until_set_raw(mask.mask);
    }

    /// Waits until a mask is set.
    #[inline(always)]
    fn wait_until_set_raw(mask: Self::T) {
        Self::wait_until(|| Self::is_set_raw(mask))
    }
}

/// A register bitmask.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Bitset<R: Register> {
    mask: R::T,
    _phantom: marker::PhantomData<R>,
}

/// A register bitmask.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Mask<R: Register> {
    mask: R::T,
    _phantom: marker::PhantomData<R>,
}

impl<R> Bitset<R> where R: Register {
    /// Creates a new register mask.
    pub const fn new(mask: R::T) -> Self {
        Bitset { mask, _phantom: marker::PhantomData }
    }

    /// Sets the mask in the register.
    ///
    /// This is equivalent to `r |= mask`.
    pub fn set_all(self) {
        R::set_raw(self.mask);
    }

    /// Clears the mask from the register.
    ///
    /// This is equivalent to `r &= !mask`.
    pub fn unset_all(self) {
        R::unset_raw(self.mask);
    }

    /// Toggles the masked bits in the register.
    ///
    /// This is equivalent to `r ^= mask`.
    pub fn toggle_all(self) {
        R::toggle_raw(self.mask);
    }

    /// Checks if the mask is clear.
    ///
    /// This is equivalent to `(r & mask) == 0`.
    pub fn is_clear(self) -> bool {
        R::is_clear_raw(self.mask)
    }
}

impl<R> Mask<R> where R: Register {
    /// Creates a new register mask.
    pub const fn new(mask: R::T) -> Self {
        Mask { mask, _phantom: marker::PhantomData }
    }

    pub fn zero() -> Self {
        Mask::new(0u8.into())
    }
}

impl<R> ops::BitOr for Mask<R> where R: Register
{
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        Mask::new(self.mask | rhs.mask)
    }
}

impl<R> ops::BitOrAssign for Mask<R> where R: Register {
    fn bitor_assign(&mut self, rhs: Self) {
        self.mask |= rhs.mask;
    }
}

impl<R> ops::BitAnd for Mask<R> where R: Register
{
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        Mask::new(self.mask & rhs.mask)
    }
}

impl<R> ops::BitAndAssign for Mask<R> where R: Register {
    fn bitand_assign(&mut self, rhs: Self) {
        self.mask &= rhs.mask;
    }
}

impl<R> ops::Not for Mask<R> where R: Register {
    type Output = Self;

    fn not(self) -> Self {
        Mask::new(!self.mask)
    }
}

impl<R> Into<u8> for Mask<R> where R: Register<T=u8> {
    fn into(self) -> u8 { self.mask }
}

impl<R> Into<u16> for Mask<R> where R: Register<T=u16> {
    fn into(self) -> u16 { self.mask }
}

impl RegisterValue for u8 { }
impl RegisterValue for u16 { }

