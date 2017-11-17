#![no_std]
#![no_main]

extern crate arduino;
use arduino::cores::current;

// Some devices may have multiple SPI modules.
// The ATmega328p only has one.
type Spi = current::Spi;

#[no_mangle]
pub extern fn main() {
}

