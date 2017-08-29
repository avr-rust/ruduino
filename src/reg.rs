use core::{cmp, convert, ops};

pub trait RegVal : Copy + Clone +
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
pub trait Register<T: RegVal> {
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
    #[inline(always)]
    fn set(mask: T) {
        unsafe {
            *Self::ADDR |= mask;
        }
    }

    /// Clears a bitmask from a register.
    #[inline(always)]
    fn unset(mask: T) {
        unsafe {
            *Self::ADDR &= !mask;
        }
    }

    /// Toggles a mask in the register.
    #[inline(always)]
    fn toggle(mask: T) {
        unsafe {
            *Self::ADDR ^= mask;
        }
    }

    /// Checks if a mask is set in the register.
    #[inline(always)]
    fn is_set(mask: T) -> bool {
        unsafe {
            (*Self::ADDR & mask) == mask
        }
    }

    /// Checks if a mask is clear in the register.
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

impl RegVal for u8 { }
impl RegVal for u16 { }

