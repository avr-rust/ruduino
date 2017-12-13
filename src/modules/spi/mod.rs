mod clock;
mod settings;

use {Register, Pin};

/// An SPI module.
///
/// Information at
/// http://maxembedded.com/2013/11/the-spi-of-the-avr/
pub trait HardwareSpi {
    type MasterInSlaveOut: Pin;
    type MasterOutSlaveIn: Pin;
    type Clock: Pin;
    type SlaveSelect: Pin;

    /// The SPI control register.
    type ControlRegister: Register<u8>;
    /// The SPI status register.
    type StatusRegister: Register<u8>;
    /// The SPI data register.
    type DataRegister: Register<u8>;

    /// Sets up the SPI as a master.
    fn setup_master(clock: u32) {
        // Setup DDR registers.
        Self::MasterInSlaveOut::set_input();
        Self::MasterOutSlaveIn::set_output();
        Self::Clock::set_output();
        Self::SlaveSelect::set_input();

        Self::set_master();
        Self::enable_interrupt();
        Self::setup_common(clock)
    }

    /// Sets up the SPI as a slave.
    fn setup_slave(clock: u32) {
        // Setup DDR registers.
        Self::MasterInSlaveOut::set_output();
        Self::MasterOutSlaveIn::set_input();
        Self::Clock::set_input();
        Self::SlaveSelect::set_input();

        Self::set_slave();
        Self::setup_common(clock)
    }

    fn setup_common(clock: u32) {
        Self::set_clock(clock);
        Self::enable()
    }

    /// Sets the clock speed.
    fn set_clock(clock: u32) {
        let mask = clock::ClockMask::with_clock(clock);
        Self::ControlRegister::set_raw(mask.control_register_mask());
        Self::StatusRegister::set_raw(mask.status_register_mask());
    }

    /// Enables interrupts for the spi module.
    #[inline(always)]
    fn enable_interrupt() {
        Self::ControlRegister::set_raw(settings::control_register::INTERRUPT_ENABLE);
    }

    /// Disables interrupts for the spi module.
    #[inline(always)]
    fn disable_interrupt() {
        Self::ControlRegister::unset_raw(settings::control_register::INTERRUPT_ENABLE);
    }

    /// Enables the SPI.
    #[inline(always)]
    fn enable() {
        Self::ControlRegister::set_raw(settings::control_register::ENABLE);
    }

    /// Disables the SPI.
    #[inline(always)]
    fn disable() {
        Self::ControlRegister::unset_raw(settings::control_register::ENABLE);
    }

    /// Enables least-significant-bit first.
    #[inline(always)]
    fn set_lsb() {
        Self::ControlRegister::set_raw(settings::control_register::DATA_ORDER_LSB);
    }

    /// Enables most-significant-bit first.
    #[inline(always)]
    fn set_msb() {
        Self::ControlRegister::unset_raw(settings::control_register::DATA_ORDER_LSB);
    }

    /// Enables master mode.
    #[inline(always)]
    fn set_master() {
        Self::ControlRegister::set_raw(settings::control_register::MASTER);
    }

    /// Enables slave mode.
    #[inline(always)]
    fn set_slave() {
        Self::ControlRegister::unset_raw(settings::control_register::MASTER);
    }

    /// Enables double speed mode.
    #[inline(always)]
    fn enable_double_speed() {
        Self::StatusRegister::set_raw(settings::status_register::SPI2X);
    }

    /// Disables double speed mode.
    #[inline(always)]
    fn disable_double_speed() {
        Self::StatusRegister::unset_raw(settings::status_register::SPI2X);
    }

    /// Checks if there is a write collision.
    #[inline(always)]
    fn is_write_collision() -> bool {
        Self::StatusRegister::is_set_raw(settings::status_register::WCOL)
    }

    /// Sends a byte through the serial.
    #[inline(always)]
    fn send_byte(byte: u8) {
        Self::DataRegister::write(byte);
        Self::StatusRegister::wait_until_set_raw(settings::status_register::SPIF);
    }

    /// Reads a byte from the serial.
    #[inline(always)]
    fn receive_byte() -> u8 {
        Self::StatusRegister::wait_until_set_raw(settings::status_register::SPIF);
        Self::DataRegister::read()
    }

    /// Sends and receives a byte.
    #[inline(always)]
    fn send_receive(byte: u8) -> u8 {
        Self::DataRegister::write(byte);
        Self::StatusRegister::wait_until_set_raw(settings::status_register::SPIF);
        Self::DataRegister::read()
    }
}

