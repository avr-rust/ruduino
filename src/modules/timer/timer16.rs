use crate::{RegisterBits, Register};
use core::marker;

/// A 16-bit timer.
pub trait Timer16 : Sized {
    /// The first compare register.
    /// For example, OCR0A.
    type CompareA: Register<T=u16>;

    /// The second compare register.
    /// For example, OCR0B.
    type CompareB: Register<T=u16>;

    /// The counter register.
    ///
    /// For example, TCNT0.
    type Counter: Register<T=u16>;

    /// The first control register.
    ///
    /// For example, TCCR0A.
    type ControlA: Register<T=u8>;

    /// The second control register.
    ///
    /// For example, TCCR0B.
    type ControlB: Register<T=u8>;

    /// The third control register.
    ///
    /// For example, TCCR0C.
    type ControlC: Register<T=u8>;

    /// The interrupt mask register.
    ///
    /// For example, TIMSK0.
    type InterruptMask: Register<T=u8>;

    /// The interrupt flag register.
    ///
    /// For example, TIFR0.
    type InterruptFlag: Register<T=u8>;

    const CS0: RegisterBits<Self::ControlB>;
    const CS1: RegisterBits<Self::ControlB>;
    const CS2: RegisterBits<Self::ControlB>;

    const WGM0: RegisterBits<Self::ControlA>;
    const WGM1: RegisterBits<Self::ControlA>;
    const WGM2: RegisterBits<Self::ControlB>;
    const WGM3: RegisterBits<Self::ControlB>;

    const OCIEA: RegisterBits<Self::InterruptMask>;

    fn setup() -> Timer16Setup<Self> { Timer16Setup::new() }
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
    fn bits<T: Timer16>(&self) -> RegisterBits<T::ControlB> {
        use self::ClockSource::*;

        match *self {
            None            => RegisterBits::zero() | RegisterBits::zero() | RegisterBits::zero(),
            Prescale1       => RegisterBits::zero() | RegisterBits::zero() | T::CS0,
            Prescale8       => RegisterBits::zero() | T::CS1       | RegisterBits::zero(),
            Prescale64      => RegisterBits::zero() | T::CS1       | T::CS0,
            Prescale256     => T::CS2       | RegisterBits::zero() | RegisterBits::zero(),
            Prescale1024    => T::CS2       | RegisterBits::zero() | T::CS0,
            ExternalFalling => T::CS2       | T::CS1       | RegisterBits::zero(),
            ExternalRising  => T::CS2       | T::CS1       | T::CS0,
        }
    }

    #[inline]
    fn mask<T: Timer16>() -> RegisterBits<T::ControlB> {
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
    fn bits<T: Timer16>(&self) -> (RegisterBits<T::ControlA>, RegisterBits<T::ControlB>) {
        use self::WaveformGenerationMode::*;
        use RegisterBits as B;

        // It makes more sense to return bytes (A,B), but the manual
        // lists the table as (B,A). We match the manual here for
        // inspection purposes and flip the values for sanity
        // purposes.
        let (b, a) = match *self {
            Normal                                   => (B::zero()    | B::zero(), B::zero() | B::zero()),
            PwmPhaseCorrect8Bit                      => (B::zero()    | B::zero(), B::zero() | T::WGM0),
            PwmPhaseCorrect9Bit                      => (B::zero()    | B::zero(), T::WGM1   | B::zero()),
            PwmPhaseCorrect10Bit                     => (B::zero()    | B::zero(), T::WGM1   | T::WGM0),
            ClearOnTimerMatchOutputCompare           => (B::zero()    | T::WGM2,   B::zero() | B::zero()),
            FastPwm8Bit                              => (B::zero()    | T::WGM2,   B::zero() | T::WGM0),
            FastPwm9Bit                              => (B::zero()    | T::WGM2,   T::WGM1   | B::zero()),
            FastPwm10Bit                             => (B::zero()    | T::WGM2,   T::WGM1   | T::WGM0),
            PwmPhaseAndFrequencyCorrectInputCapture  => (T::WGM3      | B::zero(), B::zero() | B::zero()),
            PwmPhaseAndFrequencyCorrectOutputCompare => (T::WGM3      | B::zero(), B::zero() | T::WGM0),
            PwmPhaseCorrectInputCapture              => (T::WGM3      | B::zero(), T::WGM1   | B::zero()),
            PwmPhaseCorrectOutputCompare             => (T::WGM3      | B::zero(), T::WGM1   | T::WGM0),
            ClearOnTimerMatchInputCapture            => (T::WGM3      | T::WGM2,   B::zero() | B::zero()),
            // Reserved                              => (T::WGM3      | T::WGM2,   B::zero() | T::WGM0),
            FastPwmInputCapture                      => (T::WGM3      | T::WGM2,   T::WGM1   | B::zero()),
            FastPwmOutputCompare                     => (T::WGM3      | T::WGM2,   T::WGM1   | T::WGM0),
        };

        (a, b)
    }

    #[inline]
    fn mask<T: Timer16>() -> (RegisterBits<T::ControlA>, RegisterBits<T::ControlB>) {
        (!(T::WGM0 | T::WGM1), !(T::WGM2 | T::WGM3))
    }
}

pub struct Timer16Setup<T: Timer16> {
    a: RegisterBits<T::ControlA>,
    b: RegisterBits<T::ControlB>,
    c: RegisterBits<T::ControlC>,
    output_compare_1: Option<u16>,
    _phantom: marker::PhantomData<T>,
}

impl<T: Timer16> Timer16Setup<T> {
    #[inline]
    fn new() -> Self {
        Timer16Setup {
            a: RegisterBits::zero(),
            b: RegisterBits::zero(),
            c: RegisterBits::zero(),
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
        T::ControlA::write(self.a);
        T::ControlB::write(self.b);
        T::ControlC::write(self.c);

        // Reset counter to zero
        T::Counter::write(0u16);

        if let Some(v) = self.output_compare_1 {
            // Set the match
            T::CompareA::write(v);

            // Enable compare interrupt
            T::InterruptMask::set(T::OCIEA);
        }
    }
}
