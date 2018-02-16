use config;

/// A clock mask.
///
/// The format looks like this
///
/// ```
/// 0b00000<1><0><2x>
/// ```
///
/// Where
///
/// * `1` is the value of the `SPR1` bit
/// * `0` is the value of the `SPR0` bit
/// * `2x` indicates if double speed mode is enabled
#[derive(Copy, Clone)]
pub struct ClockMask(pub u8);

impl ClockMask {
    /// Gets the clock mask for a specific baute rate.
    pub fn with_clock(spi_clock: u32) -> ClockMask {
        let mut divider_bits = if spi_clock >= config::CPU_FREQUENCY / 2 {
            0
        } else if spi_clock >= config::CPU_FREQUENCY / 4 {
            1
        } else if spi_clock >= config::CPU_FREQUENCY / 8 {
            2
        } else if spi_clock >= config::CPU_FREQUENCY / 16 {
            3
        } else if spi_clock >= config::CPU_FREQUENCY / 32 {
            4
        } else if spi_clock >= config::CPU_FREQUENCY / 64 {
            5
        } else {
            6
        };

        // Invert the SPI2X bit
        divider_bits ^= 0x1;

        // Compensate for the duplicate F_osc/64
        if divider_bits == 6 {
            divider_bits = 7;
        }
        ClockMask(divider_bits)
    }

    pub fn control_register_mask(self) -> u8 {
        // SPR1 and SPR0
        // These both form bits 1 and 0 of the control register.
        (self.0 & 0b110) >> 1
    }

    pub fn status_register_mask(self) -> u8 {
        // SPI2x
        // This forms bit 0 of the status register.
        self.0 & 0b1
    }
}

