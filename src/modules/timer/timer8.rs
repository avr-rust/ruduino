use {Mask, Register};
use core::marker;

/// A 8-bit timer.
pub trait Timer8 : Sized {
    /// The first compare register.
    /// For example, OCR0A.
    type CompareA: Register<T=u8>;

    /// The second compare register.
    /// For example, OCR0B.
    type CompareB: Register<T=u8>;

    /// The counter register.
    ///
    /// For example, TCNT0.
    type Counter: Register<T=u8>;

    /// The first control register.
    ///
    /// For example, TCCR0A.
    type ControlA: Register<T=u8>;

    /// The second control register.
    ///
    /// For example, TCCR0B.
    type ControlB: Register<T=u8>;

    /// The interrupt mask register.
    ///
    /// For example, TIMSK0.
    type InterruptMask: Register<T=u8>;

    /// The interrupt flag register.
    ///
    /// For example, TIFR0.
    type InterruptFlag: Register<T=u8>;

    const CS0: Mask<Self::ControlB>;
    const CS1: Mask<Self::ControlB>;
    const CS2: Mask<Self::ControlB>;

    const WGM0: Mask<Self::ControlA>;
    const WGM1: Mask<Self::ControlA>;
    const WGM2: Mask<Self::ControlB>;

    const OCIEA: Mask<Self::InterruptMask>;
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
    fn bits<T: Timer8>(&self) -> Mask<T::ControlB> {
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
    fn mask<T: Timer8>() -> Mask<T::ControlB> {
        !(T::CS2 | T::CS1 | T::CS0)
    }
}

pub enum WaveformGenerationMode {
    Normal,
    PwmPhaseCorrect,
    ClearOnTimerMatchOutputCompare,
    FastPwm                       ,
    PwmPhaseCorrectOutputCompare,
    FastPwmOutputCompare,
}

impl WaveformGenerationMode {
    /// Returns bits for TCCR0A, TCCR0B
    #[inline]
    fn bits<T: Timer8>(&self) -> (Mask<T::ControlA>, Mask<T::ControlB>) {
        use self::WaveformGenerationMode::*;

        // It makes more sense to return bytes (A,B), but the manual
        // lists the table as (B,A). We match the manual here for
        // inspection purposes and flip the values for sanity
        // purposes.
        let (b, a) = match *self {
            Normal                         => (Mask::zero(), Mask::zero() | Mask::zero()),
            PwmPhaseCorrect                => (Mask::zero(), Mask::zero() | T::WGM0),
            ClearOnTimerMatchOutputCompare => (Mask::zero(), T::WGM1      | Mask::zero()),
            FastPwm                        => (Mask::zero(), T::WGM1      | T::WGM0),
            // Reserved                    => (T::WGM2,      Mask::zero() | Mask::zero()),
            PwmPhaseCorrectOutputCompare   => (T::WGM2,      Mask::zero() | T::WGM0),
            // Reserved                    => (T::WGM2,      T::WGM1      | Mask::zero())),
            FastPwmOutputCompare           => (T::WGM2,      T::WGM1      | T::WGM0),
        };

        (a, b)
    }

    #[inline]
    fn mask<T: Timer8>() -> (Mask<T::ControlA>, Mask<T::ControlB>) {
        (!(T::WGM0 | T::WGM1), !(T::WGM2))
    }
}

pub struct Timer8Setup<T: Timer8> {
    a: Mask<T::ControlA>,
    b: Mask<T::ControlB>,
    output_compare_1: Option<u8>,
    _phantom: marker::PhantomData<T>,
}

impl<T: Timer8> Timer8Setup<T> {
    #[inline]
    pub fn new() -> Self {
        Timer8Setup {
            a: Mask::zero(),
            b: Mask::zero(),
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
    pub fn output_compare_1(mut self, value: Option<u8>) -> Self {
        self.output_compare_1 = value;
        self
    }

    #[inline]
    pub fn configure(self) {
        T::ControlA::write(self.a);
        T::ControlB::write(self.b);

        // Reset counter to zero
        T::Counter::write(0);

        if let Some(v) = self.output_compare_1 {
            // Set the match
            T::CompareA::write(v);

            // Enable compare interrupt
            T::InterruptMask::set(T::OCIEA);
        }
    }
}
