# Ruduino

This library provides a set of reusable components for the Arduino Uno.

## Overview

### Register and bit definitions

```rust
use ruduino::cores::current::PORTB;  // Register
use ruduino::cores::current::PORTB7; // Pin
```

### Prelude

Disable interrupts.

```rust
without_interrupts(|| {
    unsafe { write_volatile(DDRB, 0xFF) }
})
```

### Timers

Configure a timer.

```rust
const DESIRED_HZ_TIM1: f64 = 2.0;
const TIM1_PRESCALER: u64 = 1024;
const INTERRUPT_EVERY_1_HZ_1024_PRESCALER: u16 =
    ((ruduino::config::CPU_FREQUENCY_HZ as f64 / (DESIRED_HZ_TIM1 * TIM1_PRESCALER as f64)) as u64 - 1) as u16;

timer1::Timer::new()
    .waveform_generation_mode(timer1::WaveformGenerationMode::ClearOnTimerMatchOutputCompare)
    .clock_source(timer1::ClockSource::Prescale1024)
    .output_compare_1(Some(INTERRUPT_EVERY_1_HZ_1024_PRESCALER))
    .configure();
```

Set up an interrupt handler that will be called when the timer fires.

```rust
#[no_mangle]
pub unsafe extern "avr-interrupt" fn _ivr_timer1_compare_a() {
    let prev_value = read_volatile(PORTB);
    write_volatile(PORTB, prev_value ^ PINB5);
}
```

### Hardware Serial Port

Configure the serial port.

```rust
const BAUD: u64 = 9600;
const UBRR: u16 = (ruduino::config::CPU_FREQUENCY_HZ / 16 / BAUD - 1) as u16;

serial::Serial::new(UBRR)
    .character_size(serial::CharacterSize::EightBits)
    .mode(serial::Mode::Asynchronous)
    .parity(serial::Parity::Disabled)
    .stop_bits(serial::StopBits::OneBit)
    .configure();
```

Transmit a sequence of bytes.

```rust
for &b in b"OK" {
    serial::transmit(b);
}
```

Read a byte if there's something available.

```rust
if let Some(b) = serial::try_receive() {
    serial::transmit(b);
    serial::transmit(b);
}
```

