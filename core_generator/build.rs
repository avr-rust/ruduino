extern crate avr_mcu;

mod gen;

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

    let clock = env!("AVR_CPU_FREQUENCY_HZ");
    writeln!(f, "pub const CPU_FREQUENCY_HZ: u32 = {};", clock)?;
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
    writeln!(w, "use {{Mask, Register}};")?;
    writeln!(w, "use modules;")?;
    writeln!(w)?;

    gen::write_registers(mcu, w)?;
    gen::write_pins(mcu, w)?;
    gen::write_spi_modules(mcu, w)?;
    gen::write_usarts(mcu, w)?;
    gen::write_timers(mcu, w)?;

    writeln!(w)
}

