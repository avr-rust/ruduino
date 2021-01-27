extern crate avr_mcu;

mod gen;

use avr_mcu::*;
use std::fs::{self, File};
use std::io;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

/// The MCU that will be assumed when running 'cargo doc' targeting
/// archicectures that are not AVR.
const DEFAULT_MCU_FOR_NON_AVR_DOCS: &'static str = "atmega328";

// Disable core generation for various devices as it does not work for them yet.
const DISABLE_FOR_DEVICES: &'static [&'static str] = &[
    "ATmega256RFR2",
    "AT90PWM81",
    "at90can128",
    "at90can32",
    "at90can64",
    "at90pwm216",
    "at90pwm316",
    "at90usb1286",
    "at90usb1287",
    "at90usb162",
    "at90usb646",
    "at90usb647",
    "at90usb82",
    "atmega1280",
    "atmega1281",
    "atmega1284",
    "atmega1284p",
    "atmega164a",
    "atmega164p",
    "atmega164pa",
    "atmega165a",
    "atmega165p",
    "atmega165pa",
    "atmega168pb",
    "atmega169a",
    "atmega169p",
    "atmega169pa",
    "atmega16m1",
    "atmega2560",
    "atmega2561",
    "atmega324a",
    "atmega324p",
    "atmega324pa",
    "atmega324pb",
    "atmega325",
    "atmega3250",
    "atmega3250a",
    "atmega3250p",
    "atmega3250pa",
    "atmega325a",
    "atmega325p",
    "atmega325pa",
    "atmega328pb",
    "atmega329",
    "atmega3290",
    "atmega3290a",
    "atmega3290p",
    "atmega3290pa",
    "atmega329a",
    "atmega329p",
    "atmega329pa",
    "atmega32c1",
    "atmega32m1",
    "atmega48pb",
    "atmega640",
    "atmega644",
    "atmega644a",
    "atmega644p",
    "atmega644pa",
    "atmega645",
    "atmega6450",
    "atmega6450a",
    "atmega6450p",
    "atmega645a",
    "atmega645p",
    "atmega649",
    "atmega6490",
    "atmega6490a",
    "atmega6490p",
    "atmega649a",
    "atmega649p",
    "atmega64c1",
    "atmega64m1",
    "atmega88pb",
    "attiny10",
    "attiny102",
    "attiny104",
    "attiny13",
    "attiny13a",
    "attiny167",
    "attiny24",
    "attiny24a",
    "attiny4",
    "attiny43u",
    "attiny44",
    "attiny44a",
    "attiny5",
    "attiny80",
    "attiny828",
    "attiny84",
    "attiny840",
    "attiny84a",
    "attiny87",
    "attiny9",
    "at90pwm1",
    "at90pwm2b",
    "at90pwm3b",
];

fn base_output_path() -> PathBuf {
    match std::env::args().skip(1).next() {
        Some(path) => Path::new(&path).to_owned(),
        None => panic!("please pass a destination path for the generated cores on the command line"),
    }
}

fn cores_path() -> PathBuf {
    base_output_path().join("cores")
}

fn core_module_name(mcu: &Mcu) -> String {
    normalize_device_name(&mcu.device.name)
}

fn normalize_device_name(device_name: &str) -> String {
    device_name.to_lowercase().to_owned()
}

fn main() {
    if !cores_path().exists() {
        fs::create_dir_all(&cores_path()).expect("could not create cores directory");
    }

    let current_mcu = if avr_mcu::current::is_compiling_for_avr() {
        avr_mcu::current::mcu()
            .expect("no target cpu specified")
    } else {
        avr_mcu::microcontroller(DEFAULT_MCU_FOR_NON_AVR_DOCS)
    };
    let current_mcu_name = current_mcu.device.name.clone();

    let microcontrollers = avr_mcu::microcontrollers();
    let (count_total, mut cores_successful, mut cores_failed) = (microcontrollers.len(), Vec::new(), Vec::new());

    for (i, mcu) in microcontrollers.iter().enumerate() {
        if DISABLE_FOR_DEVICES.iter().any(|d| mcu.device.name == *d || core_module_name(mcu) == *d) {
            println!("skipping generation of core for '{}'", mcu.device.name);
            continue;
        }

        let result = std::panic::catch_unwind(|| {
            println!("generating core for '{}' ({} of {})", mcu.device.name, i + 1, count_total);
            generate_cores(&[mcu.clone()]).unwrap();
        });

        match result {
            Ok(..) => {
                println!("successfully generated core for '{}'", mcu.device.name);
                cores_successful.push(mcu);
            },
            Err(e) => {
                delete_core_module(mcu).unwrap(); // Don't leave around broken core files.

                let error_message = if let Some(e) = e.downcast_ref::<&dyn std::error::Error>() {
                    format!("{}", e)
                } else {
                    String::new()
                };

                eprintln!("failed to generate core for '{}', skipping: {}\n", mcu.device.name, error_message);
                cores_failed.push(mcu);
            },
        }
    }
    println!("generating 'src/cores/mod.rs' for the {} successfully generated cores", cores_successful.len());
    generate_cores_mod_rs(&cores_successful[..]).expect("failed to generates src/cores/mod.rs");

    println!("statistics:");
    println!("  total successful: {}", cores_successful.len());
    println!("  total failed: {}", cores_failed.len());


    println!("cargo:rustc-cfg=avr_mcu_{}", normalize_device_name(&current_mcu_name));
}

fn generate_cores(mcus: &[Mcu]) -> Result<(), io::Error> {
    for mcu in mcus {
        generate_core_module(mcu).expect("failed to generate mcu core");
    }

    Ok(())
}

fn generate_core_module(mcu: &Mcu) -> Result<(), io::Error> {
    let path = cores_path().join(format!("{}.rs", core_module_name(mcu)));
    let mut file = File::create(&path)?;
    write_core_module(mcu, &mut file)
}

fn delete_core_module(mcu: &Mcu) -> Result<(), io::Error> {
    let path = cores_path().join(format!("{}.rs", core_module_name(mcu)));
    std::fs::remove_file(&path)
}

fn generate_cores_mod_rs(mcus: &[&Mcu]) -> Result<(), io::Error> {
    let path = cores_path().join("mod.rs");
    let mut w = File::create(&path)?;

    writeln!(w, "//! The primary module containing microcontroller-specific core definitions")?;
    writeln!(w)?;

    for mcu in mcus {
        let module_name = core_module_name(mcu);
        writeln!(w, "/// The {}.", mcu.device.name)?;
        writeln!(w, "pub mod {};", module_name)?;

        writeln!(w, "#[cfg(avr_mcu_{})] pub use self::{} as current;", module_name, module_name)?;
        writeln!(w)?;
    }
    writeln!(w)
}

fn write_core_module(mcu: &Mcu, w: &mut dyn Write) -> Result<(), io::Error> {
    writeln!(w, "//! Core for {}.", mcu.device.name)?;
    writeln!(w)?;
    writeln!(w, "use crate::{{modules, RegisterBits, Register}};")?;
    writeln!(w)?;

    gen::write_registers(mcu, w)?;
    gen::write_pins(mcu, w)?;
    gen::write_spi_modules(mcu, w)?;
    gen::write_usarts(mcu, w)?;
    gen::write_timers(mcu, w)?;

    writeln!(w)
}

