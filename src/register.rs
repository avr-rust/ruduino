use core::{cmp, convert, marker, ops};

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
pub trait Register<T: RegisterValue> : Sized {
    type Mask = Mask<T, Self>;

    /// The address of the register.
    const ADDR: *mut T;

    /// Writes a value to the register.
    #[inline(always)]
    fn write(value: T) {
        unsafe {
            *Self::ADDR = value;
        }
    }

    /// Reads the value of the register.
    #[inline(always)]
    fn read() -> T {
        unsafe { *Self::ADDR }
    }

    /// Sets a bitmask in a register.
    ///
    /// This is equivalent to `r |= mask`.
    #[inline(always)]
    fn set(mask: T) {
        unsafe {
            *Self::ADDR |= mask;
        }
    }

    /// Clears a bitmask from a register.
    ///
    /// This is equivalent to `r &= !mask`.
    #[inline(always)]
    fn unset(mask: T) {
        unsafe {
            *Self::ADDR &= !mask;
        }
    }

    /// Toggles a mask in the register.
    ///
    /// This is equivalent to `r ^= mask`.
    #[inline(always)]
    fn toggle(mask: T) {
        unsafe {
            *Self::ADDR ^= mask;
        }
    }

    /// Checks if a mask is set in the register.
    ///
    /// This is equivalent to `(r & mask) == mask`.
    #[inline(always)]
    fn is_set(mask: T) -> bool {
        unsafe {
            (*Self::ADDR & mask) == mask
        }
    }

    /// Checks if a mask is clear in the register.
    ///
    /// This is equivalent to `(r & mask) == 0`.
    #[inline(always)]
    fn is_clear(mask: T) -> bool {
        unsafe {
            (*Self::ADDR & mask) == T::from(0)
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

    /// Waits until a mask is set.
    #[inline(always)]
    fn wait_until_set(mask: T) {
        Self::wait_until(|| Self::is_set(mask))
    }
}

/// A register bitmask.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Bitset<T: RegisterValue, R: Register<T>> {
    mask: T,
    _phantom: marker::PhantomData<R>,
}

/// A register bitmask.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Mask<T: RegisterValue, R: Register<T>> {
    mask: T,
    _phantom: marker::PhantomData<R>,
}

impl<T,R> Bitset<T,R>
    where T: RegisterValue, R: Register<T> {
    /// Creates a new register mask.
    pub const fn new(mask: T) -> Self {
        Bitset { mask, _phantom: marker::PhantomData }
    }

    /// Sets the mask in the register.
    ///
    /// This is equivalent to `r |= mask`.
    pub fn set_all(self) {
        R::set(self.mask);
    }

    /// Clears the mask from the register.
    ///
    /// This is equivalent to `r &= !mask`.
    pub fn unset_all(self) {
        R::unset(self.mask);
    }

    /// Toggles the masked bits in the register.
    ///
    /// This is equivalent to `r ^= mask`.
    pub fn toggle_all(self) {
        R::toggle(self.mask);
    }

    /// Checks if the mask is clear.
    ///
    /// This is equivalent to `(r & mask) == 0`.
    pub fn is_clear(self) -> bool {
        R::is_clear(self.mask)
    }
}

impl<T,R> Mask<T,R>
    where T: RegisterValue, R: Register<T> {
    /// Creates a new register mask.
    pub const fn new(mask: T) -> Self {
        Mask { mask, _phantom: marker::PhantomData }
    }
}

impl RegisterValue for u8 { }
impl RegisterValue for u16 { }

