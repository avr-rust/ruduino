use {DataDirection, Register};

/// An IO pin.
pub trait Pin {
    /// The associated data direction register.
    type DDR: Register<T=u8>;
    /// The associated port register.
    type PORT: Register<T=u8>;
    /// The associated pin register.
    ///
    /// Reads from the register will read input bits.
    /// Writes to the register will toggle bits.
    type PIN: Register<T=u8>;
    /// The mask of the pin used for accessing registers.
    const MASK: u8;

    /// Sets the data direction of the pin.
    #[inline(always)]
    fn set_direction(direction: DataDirection) {
        match direction {
            DataDirection::Input => Self::set_input(),
            DataDirection::Output => Self::set_output(),
        }
    }

    /// Sets the pin up as an input.
    #[inline(always)]
    fn set_input() {
        Self::DDR::unset_raw(Self::MASK);
    }

    /// Sets the pin up as an output.
    #[inline(always)]
    fn set_output() {
        Self::DDR::set_raw(Self::MASK);
    }

    /// Set the pin to high.
    ///
    /// The pin must be configured as an output.
    #[inline(always)]
    fn set_high() {
        Self::PORT::set_raw(Self::MASK);
    }

    /// Set the pin to low.
    ///
    /// The pin must be configured as an output.
    #[inline(always)]
    fn set_low() {
        Self::PORT::unset_raw(Self::MASK);
    }

    /// Toggles the pin.
    ///
    /// The pin must be configured as an output.
    #[inline(always)]
    fn toggle() {
        // FIXME: We can optimise this on post-2006 AVRs.
        // http://www.avrfreaks.net/forum/toggle-state-output-pin
        // set(Self::PIN, Self::MASK);
        Self::PORT::toggle_raw(Self::MASK);
    }

    /// Check if the pin is currently high.
    ///
    /// The pin must be configured as an input.
    #[inline(always)]
    fn is_high() -> bool {
        Self::PIN::is_set_raw(Self::MASK)
    }

    /// Checks if the pin is currently low.
    ///
    /// The pin must be configured as an input.
    #[inline(always)]
    fn is_low() -> bool {
        Self::PIN::is_clear_raw(Self::MASK)
    }
}

