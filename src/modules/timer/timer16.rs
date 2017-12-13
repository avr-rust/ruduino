use {Bitset, Mask, Register};
use core::marker;

/// A 16-bit timer.
pub trait Timer16 {
    /// The first compare register.
    /// For example, OCR0A.
    type CompareA: Register<u16>;

    /// The second compare register.
    /// For example, OCR0B.
    type CompareB: Register<u16>;

    /// The counter register.
    ///
    /// For example, TCNT0.
    type Counter: Register<u16>;

    /// The first control register.
    ///
    /// For example, TCCR0A.
    type ControlA: Register<u8>;

    /// The second control register.
    ///
    /// For example, TCCR0B.
    type ControlB: Register<u8>;

    /// The third control register.
    ///
    /// For example, TCCR0C.
    type ControlC: Register<u8>;

    /// The interrupt mask register.
    ///
    /// For example, TIMSK0.
    type InterruptMask: Register<u8>;

    /// The interrupt flag register.
    ///
    /// For example, TIFR0.
    type InterruptFlag: Register<u8>;

    const CS0: Mask<u8, Self::ControlB>;
    const CS1: Mask<u8, Self::ControlB>;
    const CS2: Mask<u8, Self::ControlB>;

    const WGM0: Mask<u8, Self::ControlA>;
    const WGM1: Mask<u8, Self::ControlA>;
    const WGM2: Mask<u8, Self::ControlB>;
    const WGM3: Mask<u8, Self::ControlB>; // fixme: right reg?

    const OCIEA: Bitset<u8, Self::InterruptMask>;
}

pub enum ClockSource {
    None,
    Prescale1,
    Prescale8,
    Prescale64,
    Prescale256,
    Prescale1024,
    ExternalFalling,
    ExternalRising,
}

impl ClockSource {
    fn bits<T: Timer16>(&self) -> Mask<u8, T::ControlB> {
        use self::ClockSource::*;

        match *self {
            None            => Mask::zero() | Mask::zero() | Mask::zero(),
            Prescale1       => Mask::zero() | Mask::zero() | T::CS0,
            Prescale8       => Mask::zero() | T::CS1       | Mask::zero(),
            Prescale64      => Mask::zero() | T::CS1       | T::CS0,
            Prescale256     => T::CS2       | Mask::zero() | Mask::zero(),
            Prescale1024    => T::CS2       | Mask::zero() | T::CS0,
            ExternalFalling => T::CS2       | T::CS1       | Mask::zero(),
            ExternalRising  => T::CS2       | T::CS1       | T::CS0,
        }
    }

    #[inline]
    fn mask<T: Timer16>() -> Mask<u8, T::ControlB> {
        !(T::CS2 | T::CS1 | T::CS0)
    }
}

pub enum WaveformGenerationMode {
    Normal,
    PwmPhaseCorrect8Bit,
    PwmPhaseCorrect9Bit,
    PwmPhaseCorrect10Bit,
    ClearOnTimerMatchOutputCompare,
    FastPwm8Bit,
    FastPwm9Bit,
    FastPwm10Bit,
    PwmPhaseAndFrequencyCorrectInputCapture,
    PwmPhaseAndFrequencyCorrectOutputCompare,
    PwmPhaseCorrectInputCapture,
    PwmPhaseCorrectOutputCompare,
    ClearOnTimerMatchInputCapture,
    FastPwmInputCapture,
    FastPwmOutputCompare,
}

impl WaveformGenerationMode {
    /// Returns bits for TCCR1A, TCCR1B
    #[inline]
    fn bits<T: Timer16>(&self) -> (Mask<u8, T::ControlA>, Mask<u8, T::ControlB>) {
        use self::WaveformGenerationMode::*;

        // It makes more sense to return bytes (A,B), but the manual
        // lists the table as (B,A). We match the manual here for
        // inspection purposes and flip the values for sanity
        // purposes.
        let (b, a) = match *self {
            Normal                                   => (Mask::zero() | Mask::zero(), Mask::zero() | Mask::zero()),
            PwmPhaseCorrect8Bit                      => (Mask::zero() | Mask::zero(), Mask::zero() | T::WGM0),
            PwmPhaseCorrect9Bit                      => (Mask::zero() | Mask::zero(), T::WGM1      | Mask::zero()),
            PwmPhaseCorrect10Bit                     => (Mask::zero() | Mask::zero(), T::WGM1      | T::WGM0),
            ClearOnTimerMatchOutputCompare           => (Mask::zero() | T::WGM2,      Mask::zero() | Mask::zero()),
            FastPwm8Bit                              => (Mask::zero() | T::WGM2,      Mask::zero() | T::WGM0),
            FastPwm9Bit                              => (Mask::zero() | T::WGM2,      T::WGM1      | Mask::zero()),
            FastPwm10Bit                             => (Mask::zero() | T::WGM2,      T::WGM1      | T::WGM0),
            PwmPhaseAndFrequencyCorrectInputCapture  => (T::WGM3      | Mask::zero(), Mask::zero() | Mask::zero()),
            PwmPhaseAndFrequencyCorrectOutputCompare => (T::WGM3      | Mask::zero(), Mask::zero() | T::WGM0),
            PwmPhaseCorrectInputCapture              => (T::WGM3      | Mask::zero(), T::WGM1      | Mask::zero()),
            PwmPhaseCorrectOutputCompare             => (T::WGM3      | Mask::zero(), T::WGM1      | T::WGM0),
            ClearOnTimerMatchInputCapture            => (T::WGM3      | T::WGM2,      Mask::zero() | Mask::zero()),
            // Reserved                              => (T::WGM3      | T::WGM2,      Mask::zero() | T::WGM0),
            FastPwmInputCapture                      => (T::WGM3      | T::WGM2,      T::WGM1      | Mask::zero()),
            FastPwmOutputCompare                     => (T::WGM3      | T::WGM2,      T::WGM1      | T::WGM0),
        };

        (a, b)
    }

    #[inline]
    fn mask<T: Timer16>() -> (Mask<u8, T::ControlA>, Mask<u8, T::ControlB>) {
        (!(T::WGM0 | T::WGM1), !(T::WGM2 | T::WGM3))
    }
}

pub struct Timer16Setup<T: Timer16> {
    a: Mask<u8, T::ControlA>,
    b: Mask<u8, T::ControlB>,
    c: Mask<u8, T::ControlC>,
    output_compare_1: Option<u16>,
    _phantom: marker::PhantomData<T>,
}

impl<T: Timer16> Timer16Setup<T> {
    #[inline]
    pub fn new() -> Self {
        Timer16Setup {
            a: Mask::zero(),
            b: Mask::zero(),
            c: Mask::zero(),
            output_compare_1: None,
            _phantom: marker::PhantomData,
        }
    }

    #[inline]
    pub fn clock_source(mut self, source: ClockSource) -> Self {
        self.b &= ClockSource::mask::<T>();
        self.b |= source.bits::<T>();
        self
    }

    #[inline]
    pub fn waveform_generation_mode(mut self, mode: WaveformGenerationMode) -> Self {
        let (a, b) = WaveformGenerationMode::mask::<T>();
        self.a &= a;
        self.b &= b;

        let (a, b) = mode.bits::<T>();
        self.a |= a;
        self.b |= b;

        self
    }

    #[inline]
    pub fn output_compare_1(mut self, value: Option<u16>) -> Self {
        self.output_compare_1 = value;
        self
    }

    #[inline]
    pub fn configure(self) {
        unsafe {
            T::ControlA::write(self.a);
            T::ControlB::write(self.b);
            T::ControlC::write(self.c);

            // Reset counter to zero
            T::Counter::write(0u16);

            if let Some(v) = self.output_compare_1 {
                // Set the match
                T::CompareA::write(v);

                // Enable compare interrupt
                // FIXME: uncomment
                // write_volatile(TIMSK1, OCIE1A);
            }
        }
    }
}
