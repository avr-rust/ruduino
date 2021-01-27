//! The primary module containing microcontroller-specific core definitions

/// The ATmega88.
pub mod atmega88;
#[cfg(avr_mcu_atmega88)] pub use self::atmega88 as current;

/// The ATmega48A.
pub mod atmega48a;
#[cfg(avr_mcu_atmega48a)] pub use self::atmega48a as current;

/// The ATmega168A.
pub mod atmega168a;
#[cfg(avr_mcu_atmega168a)] pub use self::atmega168a as current;

/// The ATmega88P.
pub mod atmega88p;
#[cfg(avr_mcu_atmega88p)] pub use self::atmega88p as current;

/// The ATmega168P.
pub mod atmega168p;
#[cfg(avr_mcu_atmega168p)] pub use self::atmega168p as current;

/// The ATmega88PA.
pub mod atmega88pa;
#[cfg(avr_mcu_atmega88pa)] pub use self::atmega88pa as current;

/// The ATmega168.
pub mod atmega168;
#[cfg(avr_mcu_atmega168)] pub use self::atmega168 as current;

/// The ATmega328P.
pub mod atmega328p;
#[cfg(avr_mcu_atmega328p)] pub use self::atmega328p as current;

/// The ATmega48PA.
pub mod atmega48pa;
#[cfg(avr_mcu_atmega48pa)] pub use self::atmega48pa as current;

/// The ATmega168PA.
pub mod atmega168pa;
#[cfg(avr_mcu_atmega168pa)] pub use self::atmega168pa as current;

/// The ATmega48P.
pub mod atmega48p;
#[cfg(avr_mcu_atmega48p)] pub use self::atmega48p as current;

/// The ATmega328.
pub mod atmega328;
#[cfg(avr_mcu_atmega328)] pub use self::atmega328 as current;

/// The ATmega88A.
pub mod atmega88a;
#[cfg(avr_mcu_atmega88a)] pub use self::atmega88a as current;

/// The ATmega48.
pub mod atmega48;
#[cfg(avr_mcu_atmega48)] pub use self::atmega48 as current;


