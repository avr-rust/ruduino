//! A serial.
//!
//! *WARNING* The current implementation of this will only work on ATmega328.

use core::prelude::v1::*;
use core::ptr::{read_volatile, write_volatile};

pub enum CharacterSize {
    FiveBits,
    SixBits,
    SevenBits,
    EightBits,
    NineBits,
}

impl CharacterSize {
    /// Returns bits for UCSR0B, UCSR0C
    #[inline]
    fn bits(&self) -> (u8, u8) {
        use self::CharacterSize::*;

        match *self {
            FiveBits    => (0     , 0      | 0     ),
            SixBits     => (0     , 0      | UCSZ00),
            SevenBits   => (0     , UCSZ01 | 0     ),
            EightBits   => (0     , UCSZ01 | UCSZ00),
            // Reserved => (UCSZ02, 0      | 0     ),
            // Reserved => (UCSZ02, 0      | UCSZ00),
            // Reserved => (UCSZ02, UCSZ01 | 0     ),
            NineBits    => (UCSZ02, UCSZ01 | UCSZ00),
        }
    }

    #[inline]
    fn mask() -> (u8, u8) {
        (!(UCSZ01 | UCSZ00), !(UCSZ02))
    }
}

pub enum Mode {
    Asynchronous,
    Synchronous,
    MasterSpi,
}

impl Mode {
    #[inline]
    fn bits(&self) -> u8 {
        use self::Mode::*;

        match *self {
            Asynchronous => 0       | 0,
            Synchronous  => 0       | UMSEL00,
            // Reserved  => UMSEL01 | 0,
            MasterSpi    => UMSEL01 | UMSEL00,
        }
    }

    #[inline]
    fn mask() -> u8 {
        !(UMSEL01 | UMSEL00)
    }
}

pub enum Parity {
    Disabled,
    Even,
    Odd,
}

impl Parity {
    #[inline]
    fn bits(&self) -> u8 {
        use self::Parity::*;

        match *self {
            Disabled    => 0     | 0,
            // Reserved => 0     | UPM00,
            Even        => UPM01 | 0,
            Odd         => UPM01 | UPM00,
        }
    }

    #[inline]
    fn mask() -> u8 {
        !(UPM01 | UPM00)
    }
}

pub enum StopBits {
    OneBit,
    TwoBits,
}

impl StopBits {
    #[inline]
    fn bits(&self) -> u8 {
        use self::StopBits::*;

        match *self {
            OneBit  => 0,
            TwoBits => USBS0,
        }
    }

    #[inline]
    fn mask() -> u8 {
        !USBS0
    }
}

/// A serial connection.
/// *WARNING* The current implementation of this will only work on ATmega328.
pub struct Serial {
    ubrr: u16,
    a: u8,
    b: u8,
    c: u8,
}

impl Serial {
    #[inline]
    pub fn new(ubrr: u16) -> Self {
        Serial {
            ubrr: ubrr,
            a: 0,
            b: 0,
            c: 0,
        }
    }

    #[inline]
    pub fn character_size(mut self, character_size: CharacterSize) -> Self {
        let (b, c) = CharacterSize::mask();
        self.b &= b;
        self.c &= c;

        let (b, c) = character_size.bits();
        self.b |= b;
        self.c |= c;

        self
    }

    #[inline]
    pub fn mode(mut self, mode: Mode) -> Self {
        self.c &= Mode::mask();
        self.c |= mode.bits();
        self
    }

    #[inline]
    pub fn parity(mut self, parity: Parity) -> Self {
        self.c &= Parity::mask();
        self.c |= parity.bits();
        self
    }

    #[inline]
    pub fn stop_bits(mut self, stop_bits: StopBits) -> Self {
        self.c &= StopBits::mask();
        self.c |= stop_bits.bits();
        self
    }

    #[inline]
    pub fn configure(self) {
        unsafe {
            // Set Baud rate
            write_volatile(UBRR0, self.ubrr);

            write_volatile(UCSR0A, self.a);
            write_volatile(UCSR0B, self.b | RXEN0 | TXEN0);
            write_volatile(UCSR0C, self.c);
        }
    }
}

#[inline]
pub fn ready_to_transmit() -> bool {
    unsafe { (read_volatile(UCSR0A) & UDRE0) != 0 }
}

#[inline]
fn do_write(byte: u8) {
    unsafe { write_volatile(UDR0, byte); }
}

/// Does a blocking transfer of one byte
#[inline]
pub fn transmit(byte: u8) {
    while !ready_to_transmit() {}
    do_write(byte);
}

#[inline]
pub fn try_transmit(byte: u8) -> Result<(), ()> {
    if ready_to_transmit() {
        do_write(byte);
        Ok(())
    } else {
        Err(())
    }
}

#[inline]
pub fn ready_to_receive() -> bool {
    unsafe { (read_volatile(UCSR0A) & RXC0) != 0 }
}

#[inline]
fn do_read() -> u8 {
    unsafe { read_volatile(UDR0) }
}

/// Does a blocking read of one byte
#[inline]
pub fn receive() -> u8 {
    while !ready_to_receive() {}
    do_read()
}

#[inline]
pub fn try_receive() -> Option<u8> {
    if ready_to_receive() {
        Some(do_read())
    } else {
        None
    }
}

// Dirty hack.
// We should write this out and use the neat build-script method instead.
use self::hack::*;
mod hack {
    macro_rules! bit {
        (-, $pos:expr) => {};
        ($name:ident, $pos:expr) => {
            pub const $name: u8 = 1 << $pos;
        };
    }

    macro_rules! register {
        ($address:expr, $name:ident, [$b7:tt, $b6:tt, $b5:tt, $b4:tt, $b3:tt, $b2:tt, $b1:tt, $b0:tt]) => {
            register!($address, $name);
            bit!($b7, 7);
            bit!($b6, 6);
            bit!($b5, 5);
            bit!($b4, 4);
            bit!($b3, 3);
            bit!($b2, 2);
            bit!($b1, 1);
            bit!($b0, 0);
        };
        ($address:expr, $name:ident) => {
            pub const $name: *mut u8 = $address as *mut u8;
        };
    }

    register!(0xC6, UDR0                                                                            );
    register!(0xC4, UBRR0L                                                                          );
    register!(0xC2, UCSR0C, [UMSEL01, UMSEL00, UPM01,   UPM00,   USBS0,   UCSZ01,  UCSZ00,  UCPOL0 ]);
    register!(0xC1, UCSR0B, [RXCIE0,  TXCIE0,  UDRIE0,  RXEN0,   TXEN0,   UCSZ02,  RXB80,   TXB80  ]);
    register!(0xC0, UCSR0A, [RXC0,    TXC0,    UDRE0,   FE0,     DOR0,    UPE0,    U2X0,    MPCM0  ]);

    // 16-bit register pairs
    pub const UBRR0: *mut u16 = UBRR0L as *mut u16;
}
