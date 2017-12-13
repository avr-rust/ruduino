//! Modules that can be implemented for specific cores.

pub use self::spi::HardwareSpi;
pub use self::timer::{Timer8, Timer8Setup};

mod spi;
mod timer;

