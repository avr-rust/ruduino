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
    fn write<V>(value: V) where V: Into<T> {
        unsafe {
            *Self::ADDR = value.into();
        }
    }

    /// Reads the value of the register.
    #[inline(always)]
    fn read() -> T {
        unsafe { *Self::ADDR }
    }

    fn set(mask: Mask<T, Self>) {
        Self::set_raw(mask.mask);
    }

    /// Sets a bitmask in a register.
    ///
    /// This is equivalent to `r |= mask`.
    #[inline(always)]
    fn set_raw(mask: T) {
        unsafe {
            *Self::ADDR |= mask;
        }
    }

    fn unset(mask: Mask<T, Self>) {
        Self::unset_raw(mask.mask);
    }

    /// Clears a bitmask from a register.
    ///
    /// This is equivalent to `r &= !mask`.
    #[inline(always)]
    fn unset_raw(mask: T) {
        unsafe {
            *Self::ADDR &= !mask;
        }
    }

    fn toggle(mask: Mask<T, Self>) {
        Self::toggle_raw(mask.mask);
    }

    /// Toggles a mask in the register.
    ///
    /// This is equivalent to `r ^= mask`.
    #[inline(always)]
    fn toggle_raw(mask: T) {
        unsafe {
            *Self::ADDR ^= mask;
        }
    }

    fn is_set(mask: Mask<T, Self>) -> bool {
        Self::is_set_raw(mask.mask)
    }

    /// Checks if a mask is set in the register.
    ///
    /// This is equivalent to `(r & mask) == mask`.
    #[inline(always)]
    fn is_set_raw(mask: T) -> bool {
        unsafe {
            (*Self::ADDR & mask) == mask
        }
    }

    fn is_clear(mask: Mask<T, Self>) -> bool {
        Self::is_clear_raw(mask.mask)
    }

    /// Checks if a mask is clear in the register.
    ///
    /// This is equivalent to `(r & mask) == 0`.
    #[inline(always)]
    fn is_clear_raw(mask: T) -> bool {
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

    fn wait_until_set(mask: Mask<T, Self>) {
        Self::wait_until_set_raw(mask.mask);
    }

    /// Waits until a mask is set.
    #[inline(always)]
    fn wait_until_set_raw(mask: T) {
        Self::wait_until(|| Self::is_set_raw(mask))
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

impl<T,R> Mask<T,R>
    where T: RegisterValue, R: Register<T> {
    /// Creates a new register mask.
    pub const fn new(mask: T) -> Self {
        Mask { mask, _phantom: marker::PhantomData }
    }

    pub fn zero() -> Self {
        Mask::new(0u8.into())
    }
}

impl<T,R> ops::BitOr for Mask<T,R>
    where T: RegisterValue, R: Register<T>
{
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        Mask::new(self.mask | rhs.mask)
    }
}

impl<T,R> ops::BitOrAssign for Mask<T,R>
    where T: RegisterValue, R: Register<T> {
    fn bitor_assign(&mut self, rhs: Self) {
        self.mask |= rhs.mask;
    }
}

impl<T,R> ops::BitAnd for Mask<T,R>
    where T: RegisterValue, R: Register<T>
{
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        Mask::new(self.mask & rhs.mask)
    }
}

impl<T,R> ops::BitAndAssign for Mask<T,R>
    where T: RegisterValue, R: Register<T> {
    fn bitand_assign(&mut self, rhs: Self) {
        self.mask &= rhs.mask;
    }
}

impl<T,R> ops::Not for Mask<T,R>
    where T: RegisterValue, R: Register<T> {
    type Output = Self;

    fn not(self) -> Self {
        Mask::new(!self.mask)
    }
}

impl<R> Into<u8> for Mask<u8,R> where R: Register<u8> {
    fn into(self) -> u8 {
        self.mask
    }
}

impl RegisterValue for u8 { }
impl RegisterValue for u16 { }

