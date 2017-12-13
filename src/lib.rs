//! Definitions of register addresses and bits within those registers

#![feature(asm)]
#![feature(associated_type_defaults)]
#![feature(const_fn)]
#![feature(lang_items)]
#![feature(unwind_attributes)]

#![no_std]

pub use self::register::{Bitset, Mask, Register, RegisterValue};
pub use self::pin::{DataDirection, Pin};

pub mod prelude;
pub mod legacy;
pub mod cores;
pub mod modules;

pub mod config;

mod register;
mod pin;
#[doc(hidden)]
pub mod std_stub;

