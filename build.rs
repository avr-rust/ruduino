const DEFAULT_MCU_FOR_NON_AVR_DOCS: &'static str = "atmega328";

fn main() {
    let current_mcu = if avr_mcu::current::is_compiling_for_avr() {
        avr_mcu::current::mcu()
            .expect("no target cpu specified")
    } else {
        avr_mcu::microcontroller(DEFAULT_MCU_FOR_NON_AVR_DOCS)
    };
    let current_mcu_name = current_mcu.device.name.clone().to_lowercase();

    println!("cargo:rustc-cfg=avr_mcu_{}", &current_mcu_name);
}
