#[cfg(target_arch = "avr")] use const_env::value_from_env;

/// The clock frequency of device being targeted in Hertz.
#[cfg(target_arch = "avr")]
pub const CPU_FREQUENCY_HZ: u32 = value_from_env!("AVR_CPU_FREQUENCY_HZ": u32);

/// The clock frequency of device being targeted in Hertz.
///
/// This value is arbitrarily set to 16MHz as this crate is currently being targeted for a non-AVR
/// device.
#[cfg(not(target_arch = "avr"))]
pub const CPU_FREQUENCY_HZ: u32 = 16_000_000; // Choose a nice default when not targeting AVR.

