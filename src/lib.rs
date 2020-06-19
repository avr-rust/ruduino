//! Definitions of register addresses and bits within those registers

#![feature(llvm_asm)]
#![feature(const_fn)]
#![feature(associated_type_defaults)]
#![feature(lang_items)]
#![feature(panic_handler)]
#![feature(unwind_attributes)]

#![no_std]

pub use self::register::{Register, RegisterBits, RegisterValue};
pub use self::pin::{DataDirection, Pin};

pub mod prelude;
pub mod legacy;
/// Low level register-based API for device-specific operations.
pub mod cores;
pub mod interrupt;
pub mod modules;

/// Configuration for the currently-targeted microcontroller.
pub mod config;

mod register;
mod pin;
#[doc(hidden)]
pub mod std_stub;

