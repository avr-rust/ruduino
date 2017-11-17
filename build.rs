extern crate avr_mcu;

use avr_mcu::*;
use std::fs::{self, File};
use std::io;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

fn src_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("src")
}

fn cores_path() -> PathBuf {
    src_path().join("cores")
}

fn core_module_name(mcu: &Mcu) -> String {
    mcu.device.name.to_lowercase().to_owned()
}

fn main() {
    if !cores_path().exists() {
        fs::create_dir_all(&cores_path()).expect("could not create cores directory");
    }

    let current_mcu = avr_mcu::current::mcu()
        .expect("no target cpu specified");
    generate_config_module().unwrap();
    generate_cores(&[current_mcu]).unwrap();
}

fn generate_cores(mcus: &[Mcu]) -> Result<(), io::Error> {
    for mcu in mcus {
        generate_core_module(mcu).expect("failed to generate mcu core");
    }
    generate_cores_mod_rs(mcus)
}

fn generate_config_module() -> Result<(), io::Error> {
    let path = src_path().join("config.rs");
    let mut f = File::create(&path)?;

    let clock = env!("AVR_CPU_FREQUENCY");
    writeln!(f, "pub const CPU_FREQUENCY: u32 = {};", clock)?;
    Ok(())
}

fn generate_core_module(mcu: &Mcu) -> Result<(), io::Error> {
    let path = cores_path().join(format!("{}.rs", core_module_name(mcu)));
    let mut file = File::create(&path)?;
    write_core_module(mcu, &mut file)
}

fn generate_cores_mod_rs(mcus: &[Mcu]) -> Result<(), io::Error> {
    let path = cores_path().join("mod.rs");
    let mut w = File::create(&path)?;

    writeln!(w, "//! Cores")?;
    writeln!(w)?;
    for mcu in mcus {
        let module_name = core_module_name(mcu);
        writeln!(w, "/// The {}.", mcu.device.name)?;
        writeln!(w, "pub mod {};", module_name)?;

        writeln!(w, "#[cfg(all(target_arch = \"avr\", target_cpu = \"{}\"))]", module_name)?;
        writeln!(w, "pub use self::{} as current;", module_name)?;
    }
    writeln!(w)
}

fn write_core_module(mcu: &Mcu, w: &mut Write) -> Result<(), io::Error> {
    writeln!(w, "//! Core for {}.", mcu.device.name)?;
    writeln!(w)?;
    writeln!(w, "use {{Mask, Bitset, HardwareUsart, Register}};")?;
    writeln!(w, "use spi::HardwareSpi;")?;
    writeln!(w)?;

    gen::write_registers(mcu, w)?;
    gen::write_pins(mcu, w)?;
    gen::write_spi_modules(mcu, w)?;
    gen::write_usarts(mcu, w)?;

    writeln!(w)
}

mod gen {
    use avr_mcu::*;
    use std::io;
    use std::io::prelude::*;

    pub fn write_registers(mcu: &Mcu, w: &mut Write) -> Result<(), io::Error> {
        for register in mcu.registers() {
            let ty = if register.size == 1 { "u8" } else { "u16" };

            // HACK: Skip, atmeg328p pack defines two of these.
            if register.name == "GTCCR" { continue; }

            writeln!(w, "pub struct {};", register.name)?;
            writeln!(w)?;

            writeln!(w, "impl {} {{", register.name)?;
            for bitfield in register.bitfields.iter() {
                // Create a mask for the whole bitset.
                writeln!(w, "    pub const {}: Bitset<{}, Self> = Bitset::new(0x{:x});", bitfield.name, ty, bitfield.mask)?;

                // We create masks for the individual bits in the field if there
                // is more than one bit in the field.
                if bitfield.mask.count_ones() > 1 {
                    let mut current_mask = bitfield.mask;
                    let mut current_mask_bit_num = 0;
                    for current_register_bit_num in 0..15 {
                        if (current_mask & 0b1) == 0b1 {
                            writeln!(w, "    pub const {}{}: Mask<{}, Self> = Mask::new(1<<{});",
                                     bitfield.name, current_mask_bit_num, ty, current_register_bit_num)?;
                            current_mask_bit_num += 1;
                        }

                        current_mask >>= 1;
                    }
                }
                writeln!(w)?;
            }
            writeln!(w, "}}")?;
            writeln!(w)?;

            writeln!(w, "impl Register<{}> for {} {{", ty, register.name)?;
            writeln!(w, "    const ADDR: *mut {} = 0x{:x} as *mut {};", ty, register.offset, ty)?;
            writeln!(w, "}}")?;
        }

        Ok(())
    }

    pub fn write_pins(mcu: &Mcu, w: &mut Write) -> Result<(), io::Error> {
        if let Some(port) = mcu.peripheral("PORT") {
            writeln!(w, "pub mod port {{")?;
            writeln!(w, "    use super::*;")?;
            writeln!(w, "    use Pin;")?;
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

    pub fn write_spi_modules(mcu: &Mcu, w: &mut Write) -> Result<(), io::Error> {
        if let Some(module) = mcu.module("SPI") {
            let peripheral = mcu.peripheral("SPI").expect("found SPI module but no peripheral");
            let port_peripheral = mcu.port_peripheral();

            writeln!(w, "pub struct Spi;")?;
            writeln!(w)?;
            writeln!(w, "impl HardwareSpi for Spi {{")?;

            for spi_signal in peripheral.signals() {
                let spi_signal_name = spi_signal.group.clone().expect("spi signal does not have group name");
                let (port_instance, port_signal) = port_peripheral.instance_signal_with_pad(&spi_signal.pad)
                    .expect("no port signal associated with the spi signal pad");
                let pin_name = self::pin_name(port_instance, port_signal);

                let const_name = match &spi_signal_name[..] {
                    "MISO" => "MasterInSlaveOut",
                    "MOSI" => "MasterOutSlaveIn",
                    "SCK" => "Clock",
                    "SS" => "SlaveSelect",
                    _ => panic!("unknown spi signal name: '{}'", spi_signal_name),
                };

                writeln!(w, "    type {} = {};", const_name, pin_name)?;
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

    pub fn write_usarts(mcu: &Mcu, w: &mut Write) -> Result<(), io::Error> {
        if let Some(module) = mcu.module("USART") {
            for usart in module.register_groups.iter() {
                writeln!(w, "/// The {} module.", usart.name)?;
                writeln!(w, "pub struct {};", usart.name)?;
                writeln!(w)?;
                writeln!(w, "impl HardwareUsart for {} {{", usart.name)?;
                for register in usart.registers.iter() {
                    let reg_ty = if register.name.starts_with("UDR") { // the data register.
                        "DataRegister".to_owned()
                    } else if register.name.starts_with("UCSR") { // one of the three control/status registers.
                        let suffix = register.name.chars().rev().next().unwrap();
                        format!("ControlRegister{}", suffix)
                    } else if register.name.starts_with("UBRR") { // the baud rate register.
                        "BaudRateRegister".to_owned()
                    } else {
                        panic!("unknown usart register '{}'", register.name);
                    };
                    writeln!(w, "    type {} = {};", reg_ty, register.name)?;
                }
                writeln!(w, "}}")?;
                writeln!(w)?;
            }
        }
        Ok(())
    }

    /// Gets the name of a pin.
    fn pin_name(instance: &Instance, signal: &Signal) -> String {
        let idx = signal.index.expect("signal with no index");
        let letter = instance.name.chars().rev().next().unwrap();
        format!("port::{}{}", letter, idx)
    }
}

