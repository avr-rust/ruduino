use {Register, Pin};


/// An SPI module.
///
/// Information at
/// http://maxembedded.com/2013/11/the-spi-of-the-avr/
pub trait HardwareSpi {
    /// Master-in slave-out pin.
    type MISO: Pin;
    /// Master-out slave-in pin.
    type MOSI: Pin;
    /// Serial clock pin.
    type SCK: Pin;
    /// Slave-select pin.
    type SS: Pin;

    /// The SPI control register.
    type SPCR: Register<u8>;
    /// The SPI status register.
    type SPSR: Register<u8>;
    /// The SPI data register.
    type SPDR: Register<u8>;

    /// Sets up the SPI as a master.
    fn setup_master() {
        // Setup DDR registers.
        Self::MISO::set_input();
        Self::MOSI::set_output();
        Self::SCK::set_output();
        Self::SS::set_input();

        Self::set_master();
        Self::enable_interrupt();
        Self::enable();
    }

    /// Sets up the SPI as a slave.
    fn setup_slave() {
        // Setup DDR registers.
        Self::MISO::set_output();
        Self::MOSI::set_input();
        Self::SCK::set_input();
        Self::SS::set_input();

        Self::set_slave();
        Self::enable();
    }

    /// Enables interrupts for the spi module.
    #[inline(always)]
    fn enable_interrupt() {
        Self::SPCR::set(spcr::INTERRUPT_ENABLE);
    }

    /// Disables interrupts for the spi module.
    #[inline(always)]
    fn disable_interrupt() {
        Self::SPCR::unset(spcr::INTERRUPT_ENABLE);
    }

    /// Enables the SPI.
    #[inline(always)]
    fn enable() {
        Self::SPCR::set(spcr::ENABLE);
    }

    /// Disables the SPI.
    #[inline(always)]
    fn disable() {
        Self::SPCR::unset(spcr::ENABLE);
    }

    /// Enables least-significant-bit first.
    #[inline(always)]
    fn set_lsb() {
        Self::SPCR::set(spcr::DATA_ORDER_LSB);
    }

    /// Enables most-significant-bit first.
    #[inline(always)]
    fn set_msb() {
        Self::SPCR::unset(spcr::DATA_ORDER_LSB);
    }

    /// Enables master mode.
    #[inline(always)]
    fn set_master() {
        Self::SPCR::set(spcr::MASTER);
    }

    /// Enables slave mode.
    #[inline(always)]
    fn set_slave() {
        Self::SPCR::unset(spcr::MASTER);
    }

    /// Enables double speed mode.
    #[inline(always)]
    fn enable_double_speed() {
        Self::SPSR::set(spsr::SPI2X);
    }

    /// Disables double speed mode.
    #[inline(always)]
    fn disable_double_speed() {
        Self::SPSR::unset(spsr::SPI2X);
    }

    /// Checks if there is a write collision.
    #[inline(always)]
    fn is_write_collision() -> bool {
        Self::SPSR::is_set(spsr::WCOL)
    }

    /// Sends a byte through the serial.
    #[inline(always)]
    fn send_byte(byte: u8) {
        Self::SPDR::write(byte);
        Self::SPSR::wait_until_set(spsr::SPIF);
    }

    /// Reads a byte from the serial.
    #[inline(always)]
    fn receive_byte() -> u8 {
        Self::SPSR::wait_until_set(spsr::SPIF);
        Self::SPDR::read()
    }

    /// Sends and receives a byte.
    #[inline(always)]
    fn send_receive(byte: u8) -> u8 {
        Self::SPDR::write(byte);
        Self::SPSR::wait_until_set(spsr::SPIF);
        Self::SPDR::read()
    }
}

/// Constants for the control register.
#[allow(dead_code)]
mod spcr {
    pub const INTERRUPT_ENABLE: u8 = 1<<7;
    pub const ENABLE: u8 = 1<<6;
    pub const DATA_ORDER_LSB: u8 = 1<<5;
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
#[allow(dead_code)]
mod spsr {
    /// SPI interrupt flag.
    pub const SPIF: u8 = 1<<7;
    /// Write collision flag.
    pub const WCOL: u8 = 1<<6;
    /// SPI double speed mode.
    pub const SPI2X: u8 = 1<<0;
}

