//! The primary module containing microcontroller-specific core definitions

/// The ATmega88.
#[cfg(any(avr_mcu_atmega88, feature = "all-mcus"))] pub mod atmega88;
#[cfg(avr_mcu_atmega88)] pub use self::atmega88 as current;

/// The ATmega48A.
#[cfg(any(avr_mcu_atmega48a, feature = "all-mcus"))] pub mod atmega48a;
#[cfg(avr_mcu_atmega48a)] pub use self::atmega48a as current;

/// The ATmega168A.
#[cfg(any(avr_mcu_atmega168a, feature = "all-mcus"))] pub mod atmega168a;
#[cfg(avr_mcu_atmega168a)] pub use self::atmega168a as current;

/// The ATmega88P.
#[cfg(any(avr_mcu_atmega88p, feature = "all-mcus"))] pub mod atmega88p;
#[cfg(avr_mcu_atmega88p)] pub use self::atmega88p as current;

/// The ATmega168P.
#[cfg(any(avr_mcu_atmega168p, feature = "all-mcus"))] pub mod atmega168p;
#[cfg(avr_mcu_atmega168p)] pub use self::atmega168p as current;

/// The ATmega88PA.
#[cfg(any(avr_mcu_atmega88pa, feature = "all-mcus"))] pub mod atmega88pa;
#[cfg(avr_mcu_atmega88pa)] pub use self::atmega88pa as current;

/// The ATmega16U4.
#[cfg(any(avr_mcu_atmega16u4, feature = "all-mcus"))] pub mod atmega16u4;
#[cfg(avr_mcu_atmega16u4)] pub use self::atmega16u4 as current;

/// The ATmega168.
#[cfg(any(avr_mcu_atmega168, feature = "all-mcus"))] pub mod atmega168;
#[cfg(avr_mcu_atmega168)] pub use self::atmega168 as current;

/// The ATmega328P.
#[cfg(any(avr_mcu_atmega328p, feature = "all-mcus"))] pub mod atmega328p;
#[cfg(avr_mcu_atmega328p)] pub use self::atmega328p as current;

/// The ATmega32U4.
#[cfg(any(avr_mcu_atmega32u4, feature = "all-mcus"))] pub mod atmega32u4;
#[cfg(avr_mcu_atmega32u4)] pub use self::atmega32u4 as current;

/// The ATmega48PA.
#[cfg(any(avr_mcu_atmega48pa, feature = "all-mcus"))] pub mod atmega48pa;
#[cfg(avr_mcu_atmega48pa)] pub use self::atmega48pa as current;

/// The ATmega168PA.
#[cfg(any(avr_mcu_atmega168pa, feature = "all-mcus"))] pub mod atmega168pa;
#[cfg(avr_mcu_atmega168pa)] pub use self::atmega168pa as current;

/// The ATmega48P.
#[cfg(any(avr_mcu_atmega48p, feature = "all-mcus"))] pub mod atmega48p;
#[cfg(avr_mcu_atmega48p)] pub use self::atmega48p as current;

/// The ATmega328.
///
/// This device is chosen as the default when the crate is targeting non-AVR devices.
#[cfg(any(avr_mcu_atmega328, feature = "all-mcus", not(target_arch = "avr")))] pub mod atmega328;
#[cfg(any(avr_mcu_atmega328, not(target_arch = "avr")))] pub use self::atmega328 as current;

/// The ATmega88A.
#[cfg(any(avr_mcu_atmega88a, feature = "all-mcus"))] pub mod atmega88a;
#[cfg(avr_mcu_atmega88a)] pub use self::atmega88a as current;

/// The ATmega48.
#[cfg(any(avr_mcu_atmega48, feature = "all-mcus"))] pub mod atmega48;
#[cfg(avr_mcu_atmega48)] pub use self::atmega48 as current;
