use super::clock::ClockMask;

#[derive(Copy, Clone)]
pub enum BitOrder {
    /// The least significant bit is sent first.
    LeastSignificantBit,
    /// The most significant bit is sent first.
    MostSignificantBit,
}

#[derive(Copy, Clone)]
pub enum ClockPhase {
    LeadingEdge,
    TrailingEdge,
}

/// SPI settings.
#[derive(Copy, Clone)]
pub struct Settings {
    /// Whether the SPI module is enabled.
    enabled: bool,
    /// Whether to be configured as a master or slave.
    master: bool,
    /// The clock speed.
    clock: u32,
    /// The bit ordering.
    bit_order: BitOrder,
    /// The clock phase.
    clock_phase: ClockPhase,
    /// Whether interrupts should be enabled.
    enable_interrupts: bool,
}

impl Settings {
    /// Gets the default settings for the master.
    pub fn master() -> Self {
        Settings {
            master: true,
            ..Default::default()
        }
    }

    /// Gets the default settings for the slave.
    pub fn slave() -> Self {
        Settings {
            master: false,
            ..Default::default()
        }
    }

    pub fn control_register_bits(self) -> u8 {
        let mut bits = 0;

        bits |= self.clock().control_register_mask();

        if self.enable_interrupts {
            bits |= control_register::INTERRUPT_ENABLE
        }
        if self.enabled {
            bits |= control_register::ENABLE
        }
        if let ClockPhase::LeadingEdge = self.clock_phase {
            bits |= control_register::CPHA;
        }

        if let BitOrder::LeastSignificantBit = self.bit_order {
            bits |= control_register::DATA_ORDER_LSB;
        }
        bits
    }

    pub fn status_register_bits(self) -> u8 {
        let mut bits = 0;

        bits |= self.clock().status_register_mask();
        bits
    }

    fn clock(self) -> ClockMask {
        ClockMask::with_clock(self.clock)
    }
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            enabled: true,
            master: true,
            // same as Arduino default in `SPI.h`.
            clock: 4_000_000,
            bit_order: BitOrder::MostSignificantBit,
            clock_phase: ClockPhase::LeadingEdge,
            enable_interrupts: false,
        }
    }
}

/// Constants for the control register.
pub mod control_register {
    /// Set if interrupts are enabled.
    pub const INTERRUPT_ENABLE: u8 = 1<<7;
    /// Set if the SPI module is enabled.
    pub const ENABLE: u8 = 1<<6;
    /// Set if data is sent in LSB format.
    pub const DATA_ORDER_LSB: u8 = 1<<5;
    /// Set if we are configuring a master.
    pub const MASTER: u8 = 1<<4;
    /// Clock polarity.
    pub const CPOL: u8 = 1<<3;
    /// Clock phase.
    pub const CPHA: u8 = 1<<2;
    /// Clock rate select 1.
    pub const SPR1: u8 = 1<<1;
    /// Clock rate select 2.
    pub const SPR0: u8 = 1<<0;
}

/// Constants for the status register.
pub mod status_register {
    /// SPI interrupt flag.
    pub const SPIF: u8 = 1<<7;
    /// Write collision flag.
    pub const WCOL: u8 = 1<<6;
    /// SPI double speed mode.
    pub const SPI2X: u8 = 1<<0;
}

