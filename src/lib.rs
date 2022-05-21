//! Definitions of register addresses and bits within those registers

#![feature(asm_experimental_arch)]
#![feature(associated_type_defaults)]
#![feature(lang_items)]
#![feature(proc_macro_hygiene)]

#![no_std]

#[cfg(feature = "avr-std-stub")] extern crate avr_std_stub;

extern crate const_env__value as const_env;

pub use self::register::{Register, RegisterBits, RegisterValue};
pub use self::pin::{DataDirection, Pin};

pub mod prelude;
pub mod legacy;
/// Low level register-based API for device-specific operations.
pub mod cores;
pub mod interrupt;
pub mod modules;

/// Configuration for the currently-targeted microcontroller.
pub use avr_config as config;

/// Delay routines.
pub use avr_delay as delay;

mod register;
mod pin;

