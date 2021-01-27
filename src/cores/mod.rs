
/// The ATmega328.
pub mod atmega328;
#[cfg(avr_mcu_atmega328)]
pub use self::atmega328 as current;

