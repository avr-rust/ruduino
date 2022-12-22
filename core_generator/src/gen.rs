use avr_mcu::*;
use std::io;
use std::io::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;

fn collect_register_bitfields<'a, I: Iterator<Item = &'a Register>>(registers: I) -> impl Iterator<Item = Register> {
    let mut canonical_registers: HashMap<String, Register> = HashMap::new();
    for register in registers {
        let k = register.name.clone();
        match canonical_registers.get_mut(&k) {
            Some(existing_register) => {
                let mut old_bitfields = HashSet::new();
                for bitfield in &existing_register.bitfields {
                    old_bitfields.insert(bitfield.name.clone());
                }
                let mut new_bitfields: Vec<Bitfield> =
                    register.bitfields.clone().into_iter().filter(|bitfield| old_bitfields.take(&bitfield.name).is_none()).collect();
                existing_register.bitfields.append(&mut new_bitfields)
            }
            None => {
                canonical_registers.insert(k, register.clone());
            }
        }
    }
    canonical_registers.into_values()
}

pub fn write_registers(mcu: &Mcu, w: &mut dyn Write) -> Result<(), io::Error> {
    for register in collect_register_bitfields(mcu.registers()) {
        let ty = if register.size == 1 { "u8" } else { "u16" };

        // HACK: Skip, atmeg328p pack defines two of these.
        if register.name == "GTCCR" { continue; }

        writeln!(w, "#[allow(non_camel_case_types)]")?;
        writeln!(w, "pub struct {};", register.name)?;
        writeln!(w)?;

        writeln!(w, "impl {} {{", register.name)?;
        for bitfield in register.bitfields.iter() {
            if !(bitfield.name.chars().all(|c| char::is_uppercase(c) || char::is_numeric(c))) { continue }

            // Create a mask for the whole bitset.
            writeln!(w, "    pub const {}: RegisterBits<Self> = RegisterBits::new(0x{:x});", bitfield.name, bitfield.mask)?;

            // We create masks for the individual bits in the field if there
            // is more than one bit in the field.
            let mut current_mask = bitfield.mask;
            let mut current_mask_bit_num = 0;
            for current_register_bit_num in 0..15 {
                if (current_mask & 0b1) == 0b1 {
                    writeln!(w, "    pub const {}{}: RegisterBits<Self> = RegisterBits::new(1<<{});",
                             bitfield.name, current_mask_bit_num, current_register_bit_num)?;
                    current_mask_bit_num += 1;
                }

                current_mask >>= 1;
            }
            writeln!(w)?;
        }
        writeln!(w, "}}")?;
        writeln!(w)?;

        writeln!(w, "impl Register for {} {{", register.name)?;
        writeln!(w, "    type T = {};", ty)?;
        writeln!(w, "    const ADDRESS: *mut {} = 0x{:x} as *mut {};", ty, register.offset, ty)?;
        writeln!(w, "}}")?;
    }

    Ok(())
}

pub fn write_pins(mcu: &Mcu, w: &mut dyn Write) -> Result<(), io::Error> {
    if let Some(port) = mcu.peripheral("PORT") {
        writeln!(w, "pub mod port {{")?;
        writeln!(w, "    #![allow(unused_imports)]")?;
        writeln!(w)?;
        writeln!(w, "    use super::*;")?;
        writeln!(w, "    use crate::Pin;")?;
        writeln!(w)?;

        for instance in port.instances.iter() {
            let port_letter = instance.name.chars().rev().next().unwrap();

            for signal in instance.signals.iter() {
                let idx = signal.index.expect("signal with no index");
                let struct_name = format!("{}{}", port_letter, idx);

                let io_module = mcu.modules.iter().find(|m| m.name == "PORT")
                    .expect("no port io module defined for this port");
                let register_group = io_module.register_groups.iter()
                    .find(|rg| rg.name == instance.name)
                    .expect("no register group defined for this port");

                writeln!(w, "    pub struct {};", struct_name)?;
                writeln!(w)?;
                writeln!(w, "    impl Pin for {} {{", struct_name)?;
                for reg in register_group.registers.iter() {
                    let mut const_name = reg.name.clone();
                    const_name.pop(); // Pop port character from register name (DDRB/PORTB/etc)..

                    writeln!(w, "        /// {}.", reg.caption)?;
                    writeln!(w, "        type {} = {};", const_name, reg.name)?;
                }
                writeln!(w, "        /// {}", signal.pad)?;
                writeln!(w, "        const MASK: u8 = 1<<{};", idx)?;
                writeln!(w, "    }}")?;
                writeln!(w)?;
            }
        }

        writeln!(w, "}}")?;
        writeln!(w)?;
    }
    Ok(())
}

pub fn write_spi_modules(mcu: &Mcu, w: &mut dyn Write) -> Result<(), io::Error> {
    if let Some(module) = mcu.module("SPI") {
        let peripheral = mcu.peripheral("SPI").expect("found SPI module but no peripheral");
        let port_peripheral = mcu.port_peripheral();

        writeln!(w, "pub struct Spi;")?;
        writeln!(w)?;
        writeln!(w, "impl modules::HardwareSpi for Spi {{")?;

        for spi_signal in peripheral.signals() {
            let spi_signal_name = spi_signal.group.clone().expect("spi signal does not have group name");
            let (port_instance, port_signal) = port_peripheral.instance_signal_with_pad(&spi_signal.pad)
                .expect("no port signal associated with the spi signal pad");
            let pin_name = self::pin_name(port_instance, port_signal);

            let const_name = match &spi_signal_name[..] {
                "MISO" => Some("MasterInSlaveOut"),
                "MOSI" => Some("MasterOutSlaveIn"),
                "SCK" => Some("Clock"),
                "SS" => Some("SlaveSelect"),
                "PDI" => None,
                "PDO" => None,
                _ => panic!("unknown spi signal name: '{}'", spi_signal_name),
            };

            if let Some(const_name) = const_name {
                writeln!(w, "    type {} = {};", const_name, pin_name)?
            }
        }

        for reg in module.registers() {
            let const_name = match &reg.caption[..] {
                "SPI Data Register" => "DataRegister",
                "SPI Status Register" => "StatusRegister",
                "SPI Control Register" => "ControlRegister",
                _ => panic!("unknown SPI module register: {}", reg.caption),
            };

            writeln!(w, "    type {} = {};", const_name, reg.name)?;
        }
        writeln!(w, "}}")?;
        writeln!(w)?;
    }
    Ok(())
}

pub fn write_usarts(mcu: &Mcu, w: &mut dyn Write) -> Result<(), io::Error> {
    if let Some(module) = mcu.module("USART") {
        for usart in module.register_groups.iter() {
            writeln!(w, "/// The {} module.", usart.name)?;
            writeln!(w, "pub struct {};", usart.name)?;
            writeln!(w)?;
            writeln!(w, "impl modules::HardwareUsart for {} {{", usart.name)?;
            for register in usart.registers.iter() {
                let reg_ty = if register.name.starts_with("UDR") { // the data register.
                    Some("DataRegister".to_owned())
                } else if register.name.starts_with("UCSR") { // one of the three control/status registers.
                    let suffix = register.name.chars().rev().next().unwrap();
                    if suffix == 'A' || suffix == 'B' || suffix == 'C' {
                        Some(format!("ControlRegister{}", suffix))
                    } else {
                        None
                    }
                } else if register.name.starts_with("UBRR") { // the baud rate register.
                    Some("BaudRateRegister".to_owned())
                } else {
                    panic!("unknown usart register '{}'", register.name);
                };
                if let Some(reg_ty) = reg_ty {
                    writeln!(w, "    type {} = {};", reg_ty, register.name)?;
                }
            }
            writeln!(w, "}}")?;
            writeln!(w)?;
        }
    }
    Ok(())
}

pub fn write_timers(mcu: &Mcu, w: &mut dyn Write) -> Result<(), io::Error> {
    if let Some(tc) = mcu.module("TC8") { // Timer/Counter, 8-bit.
        const TYPE_NAME: &'static str = "Timer8";

        let find_reg = |name: &'static str| {
            tc.registers().find(|r| r.name.starts_with(name))
                .expect(&format!("could not find '{}' register", name))
        };
        let find_reg_suffix_optional = |name: &'static str, suffix: &'static str| {
            tc.registers().find(|r| r.name.starts_with(name) && r.name.ends_with(suffix))
        };
        let find_reg_suffix = |name: &'static str, suffix: &'static str| {
            find_reg_suffix_optional(name, suffix)
                .expect(&format!("could not find '{}' register", name))
        };
        let timer_number = find_reg("TIMSK").name.chars().last().unwrap()
            .to_digit(10).unwrap();

        // TODO: At the moment, we do not support 8 bit timers that don't have two compare
        // registers.
        let should_skip_timer = find_reg_suffix_optional("OCR", "B").is_none();

        if !should_skip_timer {
            writeln!(w, "/// 8-bit timer.")?;
            writeln!(w, "pub struct {};", TYPE_NAME)?;
            writeln!(w)?;
            writeln!(w, "impl modules::Timer8 for {} {{", TYPE_NAME)?;
            writeln!(w, "    type CompareA = {};", find_reg_suffix("OCR", "A").name)?;
            writeln!(w, "    type CompareB = {};", find_reg_suffix("OCR", "B").name)?;
            writeln!(w, "    type Counter = {};", find_reg("TCNT").name)?;
            writeln!(w, "    type ControlA = {};", find_reg_suffix("TCCR", "A").name)?;
            writeln!(w, "    type ControlB = {};", find_reg_suffix("TCCR", "B").name)?;
            writeln!(w, "    type InterruptMask = {};", find_reg("TIMSK").name)?;
            writeln!(w, "    type InterruptFlag = {};", find_reg("TIFR").name)?;
            writeln!(w, "    const CS0: RegisterBits<Self::ControlB> = Self::ControlB::CS{}0;", timer_number)?;
            writeln!(w, "    const CS1: RegisterBits<Self::ControlB> = Self::ControlB::CS{}1;", timer_number)?;
            writeln!(w, "    const CS2: RegisterBits<Self::ControlB> = Self::ControlB::CS{}2;", timer_number)?;
            writeln!(w, "    const WGM0: RegisterBits<Self::ControlA> = Self::ControlA::WGM{}0;", timer_number)?;
            writeln!(w, "    const WGM1: RegisterBits<Self::ControlA> = Self::ControlA::WGM{}1;", timer_number)?;
            writeln!(w, "    const WGM2: RegisterBits<Self::ControlB> = Self::ControlB::WGM{}20;", timer_number)?;
            writeln!(w, "    const OCIEA: RegisterBits<Self::InterruptMask> = Self::InterruptMask::OCIE{}A;", timer_number)?;
            writeln!(w, "}}")?;
        }
    }

    if let Some(tc) = mcu.module("TC16") { // Timer/Counter, 16-bit.
        const TYPE_NAME: &'static str = "Timer16";

        let find_reg = |name: &'static str| {
            tc.registers().find(|r| r.name.starts_with(name))
                .expect(&format!("could not find '{}' register", name))
        };
        let find_reg_suffix = |name: &'static str, suffix: &'static str| {
            tc.registers().find(|r| r.name.starts_with(name) && r.name.ends_with(suffix))
                .expect(&format!("could not find '{}' register", name))
        };
        let timer_number = find_reg("TIMSK").name.chars().last().unwrap()
            .to_digit(10).unwrap();

        writeln!(w, "/// 16-bit timer.")?;
        writeln!(w, "pub struct {};", TYPE_NAME)?;
        writeln!(w)?;
        writeln!(w, "impl modules::Timer16 for {} {{", TYPE_NAME)?;
        writeln!(w, "    type CompareA = {};", find_reg_suffix("OCR", "A").name)?;
        writeln!(w, "    type CompareB = {};", find_reg_suffix("OCR", "B").name)?;
        writeln!(w, "    type Counter = {};", find_reg("TCNT").name)?;
        writeln!(w, "    type ControlA = {};", find_reg_suffix("TCCR", "A").name)?;
        writeln!(w, "    type ControlB = {};", find_reg_suffix("TCCR", "B").name)?;
        writeln!(w, "    type ControlC = {};", find_reg_suffix("TCCR", "C").name)?;
        writeln!(w, "    type InterruptMask = {};", find_reg("TIMSK").name)?;
        writeln!(w, "    type InterruptFlag = {};", find_reg("TIFR").name)?;
        writeln!(w, "    const CS0: RegisterBits<Self::ControlB> = Self::ControlB::CS{}0;", timer_number)?;
        writeln!(w, "    const CS1: RegisterBits<Self::ControlB> = Self::ControlB::CS{}1;", timer_number)?;
        writeln!(w, "    const CS2: RegisterBits<Self::ControlB> = Self::ControlB::CS{}2;", timer_number)?;
        writeln!(w, "    const WGM0: RegisterBits<Self::ControlA> = Self::ControlA::WGM{}0;", timer_number)?;
        writeln!(w, "    const WGM1: RegisterBits<Self::ControlA> = Self::ControlA::WGM{}1;", timer_number)?;
        writeln!(w, "    const WGM2: RegisterBits<Self::ControlB> = Self::ControlB::WGM{}0;", timer_number)?;
        writeln!(w, "    const WGM3: RegisterBits<Self::ControlB> = Self::ControlB::WGM{}1;", timer_number)?;
        writeln!(w, "    const OCIEA: RegisterBits<Self::InterruptMask> = Self::InterruptMask::OCIE{}A;", timer_number)?;
        writeln!(w, "}}")?;
    }

    Ok(())
}

/// Gets the name of a pin.
fn pin_name(instance: &Instance, signal: &Signal) -> String {
    let idx = signal.index.expect("signal with no index");
    let letter = instance.name.chars().rev().next().unwrap();
    format!("port::{}{}", letter, idx)
}
