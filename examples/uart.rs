#![no_std]
#![no_main]

//! Serial port example.
//!
//! The output is viewable with simavr
//!
//! ```
//! cargo build -Z build-std=core --target avr-atmega328p.json --examples --release
//! simavr -m atmega328p target/avr-atmega328p/release/examples/uart.elf
//! ```

use ruduino::legacy::serial;

#[no_mangle]
fn main() {
    const CPU_FREQUENCY_HZ: u64 = 16_000_000;
    const BAUD: u64 = 9600;
    const UBRR: u16 = (CPU_FREQUENCY_HZ / 16 / BAUD - 1) as u16;

    serial::Serial::new(UBRR)
        .character_size(serial::CharacterSize::EightBits)
        .mode(serial::Mode::Asynchronous)
        .parity(serial::Parity::Disabled)
        .stop_bits(serial::StopBits::OneBit)
        .configure();

    for &b in b"Hello, from Rust!\n" {
        serial::transmit(b);
    }
}
