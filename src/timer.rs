use {Register};

/// An 8-bit timer.
pub trait Timer8 {
    /// The first compare register.
    /// For example, OCR0A.
    type CompareA: Register<u8>;

    /// The second compare register.
    /// For example, OCR0B.
    type CompareB: Register<u8>;

    /// The counter register.
    ///
    /// For example, TCNT0.
    type Counter: Register<u8>;

    /// The first control register.
    ///
    /// For example, TCCR0A.
    type ControlA: Register<u8>;

    /// The second control register.
    ///
    /// For example, TCCR0B.
    type ControlB: Register<u8>;

    /// The interrupt mask register.
    ///
    /// For example, TIMSK0.
    type InterruptMask: Register<u8>;

    /// The interrupt flag register.
    ///
    /// For example, TIFR0.
    type InterruptFlag: Register<u8>;
}

