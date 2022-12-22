//! Core for ATmega32U4.

use crate::{modules, RegisterBits, Register};

#[allow(non_camel_case_types)]
pub struct OCDR;

impl OCDR {
}

impl Register for OCDR {
    type T = u8;
    const ADDRESS: *mut u8 = 0x51 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct ADC;

impl ADC {
}

impl Register for ADC {
    type T = u16;
    const ADDRESS: *mut u16 = 0x78 as *mut u16;
}
#[allow(non_camel_case_types)]
pub struct GPIOR0;

impl GPIOR0 {
    pub const GPIOR07: RegisterBits<Self> = RegisterBits::new(0x80);
    pub const GPIOR070: RegisterBits<Self> = RegisterBits::new(1<<7);

    pub const GPIOR06: RegisterBits<Self> = RegisterBits::new(0x40);
    pub const GPIOR060: RegisterBits<Self> = RegisterBits::new(1<<6);

    pub const GPIOR05: RegisterBits<Self> = RegisterBits::new(0x20);
    pub const GPIOR050: RegisterBits<Self> = RegisterBits::new(1<<5);

    pub const GPIOR04: RegisterBits<Self> = RegisterBits::new(0x10);
    pub const GPIOR040: RegisterBits<Self> = RegisterBits::new(1<<4);

    pub const GPIOR03: RegisterBits<Self> = RegisterBits::new(0x8);
    pub const GPIOR030: RegisterBits<Self> = RegisterBits::new(1<<3);

    pub const GPIOR02: RegisterBits<Self> = RegisterBits::new(0x4);
    pub const GPIOR020: RegisterBits<Self> = RegisterBits::new(1<<2);

    pub const GPIOR01: RegisterBits<Self> = RegisterBits::new(0x2);
    pub const GPIOR010: RegisterBits<Self> = RegisterBits::new(1<<1);

    pub const GPIOR00: RegisterBits<Self> = RegisterBits::new(0x1);
    pub const GPIOR000: RegisterBits<Self> = RegisterBits::new(1<<0);

}

impl Register for GPIOR0 {
    type T = u8;
    const ADDRESS: *mut u8 = 0x3e as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct OCR1C;

impl OCR1C {
}

impl Register for OCR1C {
    type T = u16;
    const ADDRESS: *mut u16 = 0x8c as *mut u16;
}
#[allow(non_camel_case_types)]
pub struct TCCR4C;

impl TCCR4C {
    pub const COM4A1S: RegisterBits<Self> = RegisterBits::new(0x80);
    pub const COM4A1S0: RegisterBits<Self> = RegisterBits::new(1<<7);

    pub const COM4A0S: RegisterBits<Self> = RegisterBits::new(0x40);
    pub const COM4A0S0: RegisterBits<Self> = RegisterBits::new(1<<6);

    pub const COM4B1S: RegisterBits<Self> = RegisterBits::new(0x20);
    pub const COM4B1S0: RegisterBits<Self> = RegisterBits::new(1<<5);

    pub const COM4B0S: RegisterBits<Self> = RegisterBits::new(0x10);
    pub const COM4B0S0: RegisterBits<Self> = RegisterBits::new(1<<4);

    pub const COM4D: RegisterBits<Self> = RegisterBits::new(0xc);
    pub const COM4D0: RegisterBits<Self> = RegisterBits::new(1<<2);
    pub const COM4D1: RegisterBits<Self> = RegisterBits::new(1<<3);

    pub const FOC4D: RegisterBits<Self> = RegisterBits::new(0x2);
    pub const FOC4D0: RegisterBits<Self> = RegisterBits::new(1<<1);

    pub const PWM4D: RegisterBits<Self> = RegisterBits::new(0x1);
    pub const PWM4D0: RegisterBits<Self> = RegisterBits::new(1<<0);

}

impl Register for TCCR4C {
    type T = u8;
    const ADDRESS: *mut u8 = 0xc2 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct TWAMR;

impl TWAMR {
    pub const TWAM: RegisterBits<Self> = RegisterBits::new(0xfe);
    pub const TWAM0: RegisterBits<Self> = RegisterBits::new(1<<1);
    pub const TWAM1: RegisterBits<Self> = RegisterBits::new(1<<2);
    pub const TWAM2: RegisterBits<Self> = RegisterBits::new(1<<3);
    pub const TWAM3: RegisterBits<Self> = RegisterBits::new(1<<4);
    pub const TWAM4: RegisterBits<Self> = RegisterBits::new(1<<5);
    pub const TWAM5: RegisterBits<Self> = RegisterBits::new(1<<6);
    pub const TWAM6: RegisterBits<Self> = RegisterBits::new(1<<7);

}

impl Register for TWAMR {
    type T = u8;
    const ADDRESS: *mut u8 = 0xbd as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct SREG;

impl SREG {
    pub const I: RegisterBits<Self> = RegisterBits::new(0x80);
    pub const I0: RegisterBits<Self> = RegisterBits::new(1<<7);

    pub const T: RegisterBits<Self> = RegisterBits::new(0x40);
    pub const T0: RegisterBits<Self> = RegisterBits::new(1<<6);

    pub const H: RegisterBits<Self> = RegisterBits::new(0x20);
    pub const H0: RegisterBits<Self> = RegisterBits::new(1<<5);

    pub const S: RegisterBits<Self> = RegisterBits::new(0x10);
    pub const S0: RegisterBits<Self> = RegisterBits::new(1<<4);

    pub const V: RegisterBits<Self> = RegisterBits::new(0x8);
    pub const V0: RegisterBits<Self> = RegisterBits::new(1<<3);

    pub const N: RegisterBits<Self> = RegisterBits::new(0x4);
    pub const N0: RegisterBits<Self> = RegisterBits::new(1<<2);

    pub const Z: RegisterBits<Self> = RegisterBits::new(0x2);
    pub const Z0: RegisterBits<Self> = RegisterBits::new(1<<1);

    pub const C: RegisterBits<Self> = RegisterBits::new(0x1);
    pub const C0: RegisterBits<Self> = RegisterBits::new(1<<0);

}

impl Register for SREG {
    type T = u8;
    const ADDRESS: *mut u8 = 0x5f as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct OCR3C;

impl OCR3C {
}

impl Register for OCR3C {
    type T = u16;
    const ADDRESS: *mut u16 = 0x9c as *mut u16;
}
#[allow(non_camel_case_types)]
pub struct TCCR4A;

impl TCCR4A {
    pub const COM4A: RegisterBits<Self> = RegisterBits::new(0xc0);
    pub const COM4A0: RegisterBits<Self> = RegisterBits::new(1<<6);
    pub const COM4A1: RegisterBits<Self> = RegisterBits::new(1<<7);

    pub const COM4B: RegisterBits<Self> = RegisterBits::new(0x30);
    pub const COM4B0: RegisterBits<Self> = RegisterBits::new(1<<4);
    pub const COM4B1: RegisterBits<Self> = RegisterBits::new(1<<5);

    pub const FOC4A: RegisterBits<Self> = RegisterBits::new(0x8);
    pub const FOC4A0: RegisterBits<Self> = RegisterBits::new(1<<3);

    pub const FOC4B: RegisterBits<Self> = RegisterBits::new(0x4);
    pub const FOC4B0: RegisterBits<Self> = RegisterBits::new(1<<2);

    pub const PWM4A: RegisterBits<Self> = RegisterBits::new(0x2);
    pub const PWM4A0: RegisterBits<Self> = RegisterBits::new(1<<1);

    pub const PWM4B: RegisterBits<Self> = RegisterBits::new(0x1);
    pub const PWM4B0: RegisterBits<Self> = RegisterBits::new(1<<0);

}

impl Register for TCCR4A {
    type T = u8;
    const ADDRESS: *mut u8 = 0xc0 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct CLKSEL1;

impl CLKSEL1 {
    pub const RCCKSEL: RegisterBits<Self> = RegisterBits::new(0xf0);
    pub const RCCKSEL0: RegisterBits<Self> = RegisterBits::new(1<<4);
    pub const RCCKSEL1: RegisterBits<Self> = RegisterBits::new(1<<5);
    pub const RCCKSEL2: RegisterBits<Self> = RegisterBits::new(1<<6);
    pub const RCCKSEL3: RegisterBits<Self> = RegisterBits::new(1<<7);

    pub const EXCKSEL: RegisterBits<Self> = RegisterBits::new(0xf);
    pub const EXCKSEL0: RegisterBits<Self> = RegisterBits::new(1<<0);
    pub const EXCKSEL1: RegisterBits<Self> = RegisterBits::new(1<<1);
    pub const EXCKSEL2: RegisterBits<Self> = RegisterBits::new(1<<2);
    pub const EXCKSEL3: RegisterBits<Self> = RegisterBits::new(1<<3);

}

impl Register for CLKSEL1 {
    type T = u8;
    const ADDRESS: *mut u8 = 0xc6 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct UDIEN;

impl UDIEN {
    pub const UPRSME: RegisterBits<Self> = RegisterBits::new(0x40);
    pub const UPRSME0: RegisterBits<Self> = RegisterBits::new(1<<6);

    pub const EORSME: RegisterBits<Self> = RegisterBits::new(0x20);
    pub const EORSME0: RegisterBits<Self> = RegisterBits::new(1<<5);

    pub const WAKEUPE: RegisterBits<Self> = RegisterBits::new(0x10);
    pub const WAKEUPE0: RegisterBits<Self> = RegisterBits::new(1<<4);

    pub const EORSTE: RegisterBits<Self> = RegisterBits::new(0x8);
    pub const EORSTE0: RegisterBits<Self> = RegisterBits::new(1<<3);

    pub const SOFE: RegisterBits<Self> = RegisterBits::new(0x4);
    pub const SOFE0: RegisterBits<Self> = RegisterBits::new(1<<2);

    pub const SUSPE: RegisterBits<Self> = RegisterBits::new(0x1);
    pub const SUSPE0: RegisterBits<Self> = RegisterBits::new(1<<0);

}

impl Register for UDIEN {
    type T = u8;
    const ADDRESS: *mut u8 = 0xe2 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct TCCR3C;

impl TCCR3C {
    pub const FOC3A: RegisterBits<Self> = RegisterBits::new(0x80);
    pub const FOC3A0: RegisterBits<Self> = RegisterBits::new(1<<7);

    pub const FOC3B: RegisterBits<Self> = RegisterBits::new(0x40);
    pub const FOC3B0: RegisterBits<Self> = RegisterBits::new(1<<6);

    pub const FOC3C: RegisterBits<Self> = RegisterBits::new(0x20);
    pub const FOC3C0: RegisterBits<Self> = RegisterBits::new(1<<5);

}

impl Register for TCCR3C {
    type T = u8;
    const ADDRESS: *mut u8 = 0x92 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct TCNT0;

impl TCNT0 {
}

impl Register for TCNT0 {
    type T = u8;
    const ADDRESS: *mut u8 = 0x46 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct EEAR;

impl EEAR {
}

impl Register for EEAR {
    type T = u16;
    const ADDRESS: *mut u16 = 0x41 as *mut u16;
}
#[allow(non_camel_case_types)]
pub struct EEDR;

impl EEDR {
}

impl Register for EEDR {
    type T = u8;
    const ADDRESS: *mut u8 = 0x40 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct UECFG1X;

impl UECFG1X {
    pub const EPSIZE: RegisterBits<Self> = RegisterBits::new(0x70);
    pub const EPSIZE0: RegisterBits<Self> = RegisterBits::new(1<<4);
    pub const EPSIZE1: RegisterBits<Self> = RegisterBits::new(1<<5);
    pub const EPSIZE2: RegisterBits<Self> = RegisterBits::new(1<<6);

    pub const EPBK: RegisterBits<Self> = RegisterBits::new(0xc);
    pub const EPBK0: RegisterBits<Self> = RegisterBits::new(1<<2);
    pub const EPBK1: RegisterBits<Self> = RegisterBits::new(1<<3);

    pub const ALLOC: RegisterBits<Self> = RegisterBits::new(0x2);
    pub const ALLOC0: RegisterBits<Self> = RegisterBits::new(1<<1);

}

impl Register for UECFG1X {
    type T = u8;
    const ADDRESS: *mut u8 = 0xed as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct TCCR3A;

impl TCCR3A {
    pub const COM3A: RegisterBits<Self> = RegisterBits::new(0xc0);
    pub const COM3A0: RegisterBits<Self> = RegisterBits::new(1<<6);
    pub const COM3A1: RegisterBits<Self> = RegisterBits::new(1<<7);

    pub const COM3B: RegisterBits<Self> = RegisterBits::new(0x30);
    pub const COM3B0: RegisterBits<Self> = RegisterBits::new(1<<4);
    pub const COM3B1: RegisterBits<Self> = RegisterBits::new(1<<5);

    pub const COM3C: RegisterBits<Self> = RegisterBits::new(0xc);
    pub const COM3C0: RegisterBits<Self> = RegisterBits::new(1<<2);
    pub const COM3C1: RegisterBits<Self> = RegisterBits::new(1<<3);

    pub const WGM3: RegisterBits<Self> = RegisterBits::new(0x3);
    pub const WGM30: RegisterBits<Self> = RegisterBits::new(1<<0);
    pub const WGM31: RegisterBits<Self> = RegisterBits::new(1<<1);

}

impl Register for TCCR3A {
    type T = u8;
    const ADDRESS: *mut u8 = 0x90 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct DDRB;

impl DDRB {
}

impl Register for DDRB {
    type T = u8;
    const ADDRESS: *mut u8 = 0x24 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct PORTF;

impl PORTF {
}

impl Register for PORTF {
    type T = u8;
    const ADDRESS: *mut u8 = 0x31 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct UDFNUM;

impl UDFNUM {
}

impl Register for UDFNUM {
    type T = u16;
    const ADDRESS: *mut u16 = 0xe4 as *mut u16;
}
#[allow(non_camel_case_types)]
pub struct TWDR;

impl TWDR {
}

impl Register for TWDR {
    type T = u8;
    const ADDRESS: *mut u8 = 0xbb as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct USBCON;

impl USBCON {
    pub const USBE: RegisterBits<Self> = RegisterBits::new(0x80);
    pub const USBE0: RegisterBits<Self> = RegisterBits::new(1<<7);

    pub const FRZCLK: RegisterBits<Self> = RegisterBits::new(0x20);
    pub const FRZCLK0: RegisterBits<Self> = RegisterBits::new(1<<5);

    pub const OTGPADE: RegisterBits<Self> = RegisterBits::new(0x10);
    pub const OTGPADE0: RegisterBits<Self> = RegisterBits::new(1<<4);

    pub const VBUSTE: RegisterBits<Self> = RegisterBits::new(0x1);
    pub const VBUSTE0: RegisterBits<Self> = RegisterBits::new(1<<0);

}

impl Register for USBCON {
    type T = u8;
    const ADDRESS: *mut u8 = 0xd8 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct TIMSK3;

impl TIMSK3 {
    pub const ICIE3: RegisterBits<Self> = RegisterBits::new(0x20);
    pub const ICIE30: RegisterBits<Self> = RegisterBits::new(1<<5);

    pub const OCIE3C: RegisterBits<Self> = RegisterBits::new(0x8);
    pub const OCIE3C0: RegisterBits<Self> = RegisterBits::new(1<<3);

    pub const OCIE3B: RegisterBits<Self> = RegisterBits::new(0x4);
    pub const OCIE3B0: RegisterBits<Self> = RegisterBits::new(1<<2);

    pub const OCIE3A: RegisterBits<Self> = RegisterBits::new(0x2);
    pub const OCIE3A0: RegisterBits<Self> = RegisterBits::new(1<<1);

    pub const TOIE3: RegisterBits<Self> = RegisterBits::new(0x1);
    pub const TOIE30: RegisterBits<Self> = RegisterBits::new(1<<0);

}

impl Register for TIMSK3 {
    type T = u8;
    const ADDRESS: *mut u8 = 0x71 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct SPDR;

impl SPDR {
}

impl Register for SPDR {
    type T = u8;
    const ADDRESS: *mut u8 = 0x4e as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct TCCR3B;

impl TCCR3B {
    pub const ICNC3: RegisterBits<Self> = RegisterBits::new(0x80);
    pub const ICNC30: RegisterBits<Self> = RegisterBits::new(1<<7);

    pub const ICES3: RegisterBits<Self> = RegisterBits::new(0x40);
    pub const ICES30: RegisterBits<Self> = RegisterBits::new(1<<6);

    pub const WGM3: RegisterBits<Self> = RegisterBits::new(0x18);
    pub const WGM30: RegisterBits<Self> = RegisterBits::new(1<<3);
    pub const WGM31: RegisterBits<Self> = RegisterBits::new(1<<4);

    pub const CS3: RegisterBits<Self> = RegisterBits::new(0x7);
    pub const CS30: RegisterBits<Self> = RegisterBits::new(1<<0);
    pub const CS31: RegisterBits<Self> = RegisterBits::new(1<<1);
    pub const CS32: RegisterBits<Self> = RegisterBits::new(1<<2);

}

impl Register for TCCR3B {
    type T = u8;
    const ADDRESS: *mut u8 = 0x91 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct TCCR4D;

impl TCCR4D {
    pub const FPIE4: RegisterBits<Self> = RegisterBits::new(0x80);
    pub const FPIE40: RegisterBits<Self> = RegisterBits::new(1<<7);

    pub const FPEN4: RegisterBits<Self> = RegisterBits::new(0x40);
    pub const FPEN40: RegisterBits<Self> = RegisterBits::new(1<<6);

    pub const FPNC4: RegisterBits<Self> = RegisterBits::new(0x20);
    pub const FPNC40: RegisterBits<Self> = RegisterBits::new(1<<5);

    pub const FPES4: RegisterBits<Self> = RegisterBits::new(0x10);
    pub const FPES40: RegisterBits<Self> = RegisterBits::new(1<<4);

    pub const FPAC4: RegisterBits<Self> = RegisterBits::new(0x8);
    pub const FPAC40: RegisterBits<Self> = RegisterBits::new(1<<3);

    pub const FPF4: RegisterBits<Self> = RegisterBits::new(0x4);
    pub const FPF40: RegisterBits<Self> = RegisterBits::new(1<<2);

    pub const WGM4: RegisterBits<Self> = RegisterBits::new(0x3);
    pub const WGM40: RegisterBits<Self> = RegisterBits::new(1<<0);
    pub const WGM41: RegisterBits<Self> = RegisterBits::new(1<<1);

}

impl Register for TCCR4D {
    type T = u8;
    const ADDRESS: *mut u8 = 0xc3 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct OCR4C;

impl OCR4C {
}

impl Register for OCR4C {
    type T = u8;
    const ADDRESS: *mut u8 = 0xd1 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct SPSR;

impl SPSR {
    pub const SPIF: RegisterBits<Self> = RegisterBits::new(0x80);
    pub const SPIF0: RegisterBits<Self> = RegisterBits::new(1<<7);

    pub const WCOL: RegisterBits<Self> = RegisterBits::new(0x40);
    pub const WCOL0: RegisterBits<Self> = RegisterBits::new(1<<6);

    pub const SPI2X: RegisterBits<Self> = RegisterBits::new(0x1);
    pub const SPI2X0: RegisterBits<Self> = RegisterBits::new(1<<0);

}

impl Register for SPSR {
    type T = u8;
    const ADDRESS: *mut u8 = 0x4d as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct TIFR0;

impl TIFR0 {
    pub const OCF0B: RegisterBits<Self> = RegisterBits::new(0x4);
    pub const OCF0B0: RegisterBits<Self> = RegisterBits::new(1<<2);

    pub const OCF0A: RegisterBits<Self> = RegisterBits::new(0x2);
    pub const OCF0A0: RegisterBits<Self> = RegisterBits::new(1<<1);

    pub const TOV0: RegisterBits<Self> = RegisterBits::new(0x1);
    pub const TOV00: RegisterBits<Self> = RegisterBits::new(1<<0);

}

impl Register for TIFR0 {
    type T = u8;
    const ADDRESS: *mut u8 = 0x35 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct PCMSK0;

impl PCMSK0 {
}

impl Register for PCMSK0 {
    type T = u8;
    const ADDRESS: *mut u8 = 0x6b as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct UESTA0X;

impl UESTA0X {
    pub const CFGOK: RegisterBits<Self> = RegisterBits::new(0x80);
    pub const CFGOK0: RegisterBits<Self> = RegisterBits::new(1<<7);

    pub const OVERFI: RegisterBits<Self> = RegisterBits::new(0x40);
    pub const OVERFI0: RegisterBits<Self> = RegisterBits::new(1<<6);

    pub const UNDERFI: RegisterBits<Self> = RegisterBits::new(0x20);
    pub const UNDERFI0: RegisterBits<Self> = RegisterBits::new(1<<5);

    pub const DTSEQ: RegisterBits<Self> = RegisterBits::new(0xc);
    pub const DTSEQ0: RegisterBits<Self> = RegisterBits::new(1<<2);
    pub const DTSEQ1: RegisterBits<Self> = RegisterBits::new(1<<3);

    pub const NBUSYBK: RegisterBits<Self> = RegisterBits::new(0x3);
    pub const NBUSYBK0: RegisterBits<Self> = RegisterBits::new(1<<0);
    pub const NBUSYBK1: RegisterBits<Self> = RegisterBits::new(1<<1);

}

impl Register for UESTA0X {
    type T = u8;
    const ADDRESS: *mut u8 = 0xee as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct UDCON;

impl UDCON {
    pub const LSM: RegisterBits<Self> = RegisterBits::new(0x4);
    pub const LSM0: RegisterBits<Self> = RegisterBits::new(1<<2);

    pub const RSTCPU: RegisterBits<Self> = RegisterBits::new(0x8);
    pub const RSTCPU0: RegisterBits<Self> = RegisterBits::new(1<<3);

    pub const RMWKUP: RegisterBits<Self> = RegisterBits::new(0x2);
    pub const RMWKUP0: RegisterBits<Self> = RegisterBits::new(1<<1);

    pub const DETACH: RegisterBits<Self> = RegisterBits::new(0x1);
    pub const DETACH0: RegisterBits<Self> = RegisterBits::new(1<<0);

}

impl Register for UDCON {
    type T = u8;
    const ADDRESS: *mut u8 = 0xe0 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct TWSR;

impl TWSR {
    pub const TWS: RegisterBits<Self> = RegisterBits::new(0xf8);
    pub const TWS0: RegisterBits<Self> = RegisterBits::new(1<<3);
    pub const TWS1: RegisterBits<Self> = RegisterBits::new(1<<4);
    pub const TWS2: RegisterBits<Self> = RegisterBits::new(1<<5);
    pub const TWS3: RegisterBits<Self> = RegisterBits::new(1<<6);
    pub const TWS4: RegisterBits<Self> = RegisterBits::new(1<<7);

    pub const TWPS: RegisterBits<Self> = RegisterBits::new(0x3);
    pub const TWPS0: RegisterBits<Self> = RegisterBits::new(1<<0);
    pub const TWPS1: RegisterBits<Self> = RegisterBits::new(1<<1);

}

impl Register for TWSR {
    type T = u8;
    const ADDRESS: *mut u8 = 0xb9 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct CLKSTA;

impl CLKSTA {
    pub const RCON: RegisterBits<Self> = RegisterBits::new(0x2);
    pub const RCON0: RegisterBits<Self> = RegisterBits::new(1<<1);

    pub const EXTON: RegisterBits<Self> = RegisterBits::new(0x1);
    pub const EXTON0: RegisterBits<Self> = RegisterBits::new(1<<0);

}

impl Register for CLKSTA {
    type T = u8;
    const ADDRESS: *mut u8 = 0xc7 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct ICR1;

impl ICR1 {
}

impl Register for ICR1 {
    type T = u16;
    const ADDRESS: *mut u16 = 0x86 as *mut u16;
}
#[allow(non_camel_case_types)]
pub struct TCCR1B;

impl TCCR1B {
    pub const ICNC1: RegisterBits<Self> = RegisterBits::new(0x80);
    pub const ICNC10: RegisterBits<Self> = RegisterBits::new(1<<7);

    pub const ICES1: RegisterBits<Self> = RegisterBits::new(0x40);
    pub const ICES10: RegisterBits<Self> = RegisterBits::new(1<<6);

    pub const WGM1: RegisterBits<Self> = RegisterBits::new(0x18);
    pub const WGM10: RegisterBits<Self> = RegisterBits::new(1<<3);
    pub const WGM11: RegisterBits<Self> = RegisterBits::new(1<<4);

    pub const CS1: RegisterBits<Self> = RegisterBits::new(0x7);
    pub const CS10: RegisterBits<Self> = RegisterBits::new(1<<0);
    pub const CS11: RegisterBits<Self> = RegisterBits::new(1<<1);
    pub const CS12: RegisterBits<Self> = RegisterBits::new(1<<2);

}

impl Register for TCCR1B {
    type T = u8;
    const ADDRESS: *mut u8 = 0x81 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct DDRE;

impl DDRE {
}

impl Register for DDRE {
    type T = u8;
    const ADDRESS: *mut u8 = 0x2d as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct TIFR1;

impl TIFR1 {
    pub const ICF1: RegisterBits<Self> = RegisterBits::new(0x20);
    pub const ICF10: RegisterBits<Self> = RegisterBits::new(1<<5);

    pub const OCF1C: RegisterBits<Self> = RegisterBits::new(0x8);
    pub const OCF1C0: RegisterBits<Self> = RegisterBits::new(1<<3);

    pub const OCF1B: RegisterBits<Self> = RegisterBits::new(0x4);
    pub const OCF1B0: RegisterBits<Self> = RegisterBits::new(1<<2);

    pub const OCF1A: RegisterBits<Self> = RegisterBits::new(0x2);
    pub const OCF1A0: RegisterBits<Self> = RegisterBits::new(1<<1);

    pub const TOV1: RegisterBits<Self> = RegisterBits::new(0x1);
    pub const TOV10: RegisterBits<Self> = RegisterBits::new(1<<0);

}

impl Register for TIFR1 {
    type T = u8;
    const ADDRESS: *mut u8 = 0x36 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct SP;

impl SP {
}

impl Register for SP {
    type T = u16;
    const ADDRESS: *mut u16 = 0x5d as *mut u16;
}
#[allow(non_camel_case_types)]
pub struct LOCKBIT;

impl LOCKBIT {
    pub const LB: RegisterBits<Self> = RegisterBits::new(0x3);
    pub const LB0: RegisterBits<Self> = RegisterBits::new(1<<0);
    pub const LB1: RegisterBits<Self> = RegisterBits::new(1<<1);

    pub const BLB0: RegisterBits<Self> = RegisterBits::new(0xc);
    pub const BLB00: RegisterBits<Self> = RegisterBits::new(1<<2);
    pub const BLB01: RegisterBits<Self> = RegisterBits::new(1<<3);

    pub const BLB1: RegisterBits<Self> = RegisterBits::new(0x30);
    pub const BLB10: RegisterBits<Self> = RegisterBits::new(1<<4);
    pub const BLB11: RegisterBits<Self> = RegisterBits::new(1<<5);

}

impl Register for LOCKBIT {
    type T = u8;
    const ADDRESS: *mut u8 = 0x0 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct OSCCAL;

impl OSCCAL {
    pub const OSCCAL: RegisterBits<Self> = RegisterBits::new(0xff);
    pub const OSCCAL0: RegisterBits<Self> = RegisterBits::new(1<<0);
    pub const OSCCAL1: RegisterBits<Self> = RegisterBits::new(1<<1);
    pub const OSCCAL2: RegisterBits<Self> = RegisterBits::new(1<<2);
    pub const OSCCAL3: RegisterBits<Self> = RegisterBits::new(1<<3);
    pub const OSCCAL4: RegisterBits<Self> = RegisterBits::new(1<<4);
    pub const OSCCAL5: RegisterBits<Self> = RegisterBits::new(1<<5);
    pub const OSCCAL6: RegisterBits<Self> = RegisterBits::new(1<<6);
    pub const OSCCAL7: RegisterBits<Self> = RegisterBits::new(1<<7);

}

impl Register for OSCCAL {
    type T = u8;
    const ADDRESS: *mut u8 = 0x66 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct OCR0A;

impl OCR0A {
}

impl Register for OCR0A {
    type T = u8;
    const ADDRESS: *mut u8 = 0x47 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct TCNT1;

impl TCNT1 {
}

impl Register for TCNT1 {
    type T = u16;
    const ADDRESS: *mut u16 = 0x84 as *mut u16;
}
#[allow(non_camel_case_types)]
pub struct MCUSR;

impl MCUSR {
    pub const JTRF: RegisterBits<Self> = RegisterBits::new(0x10);
    pub const JTRF0: RegisterBits<Self> = RegisterBits::new(1<<4);

    pub const WDRF: RegisterBits<Self> = RegisterBits::new(0x8);
    pub const WDRF0: RegisterBits<Self> = RegisterBits::new(1<<3);

    pub const BORF: RegisterBits<Self> = RegisterBits::new(0x4);
    pub const BORF0: RegisterBits<Self> = RegisterBits::new(1<<2);

    pub const EXTRF: RegisterBits<Self> = RegisterBits::new(0x2);
    pub const EXTRF0: RegisterBits<Self> = RegisterBits::new(1<<1);

    pub const PORF: RegisterBits<Self> = RegisterBits::new(0x1);
    pub const PORF0: RegisterBits<Self> = RegisterBits::new(1<<0);

}

impl Register for MCUSR {
    type T = u8;
    const ADDRESS: *mut u8 = 0x54 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct CLKPR;

impl CLKPR {
    pub const CLKPCE: RegisterBits<Self> = RegisterBits::new(0x80);
    pub const CLKPCE0: RegisterBits<Self> = RegisterBits::new(1<<7);

    pub const CLKPS: RegisterBits<Self> = RegisterBits::new(0xf);
    pub const CLKPS0: RegisterBits<Self> = RegisterBits::new(1<<0);
    pub const CLKPS1: RegisterBits<Self> = RegisterBits::new(1<<1);
    pub const CLKPS2: RegisterBits<Self> = RegisterBits::new(1<<2);
    pub const CLKPS3: RegisterBits<Self> = RegisterBits::new(1<<3);

}

impl Register for CLKPR {
    type T = u8;
    const ADDRESS: *mut u8 = 0x61 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct TCCR4B;

impl TCCR4B {
    pub const PWM4X: RegisterBits<Self> = RegisterBits::new(0x80);
    pub const PWM4X0: RegisterBits<Self> = RegisterBits::new(1<<7);

    pub const PSR4: RegisterBits<Self> = RegisterBits::new(0x40);
    pub const PSR40: RegisterBits<Self> = RegisterBits::new(1<<6);

    pub const DTPS4: RegisterBits<Self> = RegisterBits::new(0x30);
    pub const DTPS40: RegisterBits<Self> = RegisterBits::new(1<<4);
    pub const DTPS41: RegisterBits<Self> = RegisterBits::new(1<<5);

    pub const CS4: RegisterBits<Self> = RegisterBits::new(0xf);
    pub const CS40: RegisterBits<Self> = RegisterBits::new(1<<0);
    pub const CS41: RegisterBits<Self> = RegisterBits::new(1<<1);
    pub const CS42: RegisterBits<Self> = RegisterBits::new(1<<2);
    pub const CS43: RegisterBits<Self> = RegisterBits::new(1<<3);

}

impl Register for TCCR4B {
    type T = u8;
    const ADDRESS: *mut u8 = 0xc1 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct DIDR2;

impl DIDR2 {
    pub const ADC13D: RegisterBits<Self> = RegisterBits::new(0x20);
    pub const ADC13D0: RegisterBits<Self> = RegisterBits::new(1<<5);

    pub const ADC12D: RegisterBits<Self> = RegisterBits::new(0x10);
    pub const ADC12D0: RegisterBits<Self> = RegisterBits::new(1<<4);

    pub const ADC11D: RegisterBits<Self> = RegisterBits::new(0x8);
    pub const ADC11D0: RegisterBits<Self> = RegisterBits::new(1<<3);

    pub const ADC10D: RegisterBits<Self> = RegisterBits::new(0x4);
    pub const ADC10D0: RegisterBits<Self> = RegisterBits::new(1<<2);

    pub const ADC9D: RegisterBits<Self> = RegisterBits::new(0x2);
    pub const ADC9D0: RegisterBits<Self> = RegisterBits::new(1<<1);

    pub const ADC8D: RegisterBits<Self> = RegisterBits::new(0x1);
    pub const ADC8D0: RegisterBits<Self> = RegisterBits::new(1<<0);

}

impl Register for DIDR2 {
    type T = u8;
    const ADDRESS: *mut u8 = 0x7d as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct PRR0;

impl PRR0 {
    pub const PRTWI: RegisterBits<Self> = RegisterBits::new(0x80);
    pub const PRTWI0: RegisterBits<Self> = RegisterBits::new(1<<7);

    pub const PRTIM2: RegisterBits<Self> = RegisterBits::new(0x40);
    pub const PRTIM20: RegisterBits<Self> = RegisterBits::new(1<<6);

    pub const PRTIM0: RegisterBits<Self> = RegisterBits::new(0x20);
    pub const PRTIM00: RegisterBits<Self> = RegisterBits::new(1<<5);

    pub const PRTIM1: RegisterBits<Self> = RegisterBits::new(0x8);
    pub const PRTIM10: RegisterBits<Self> = RegisterBits::new(1<<3);

    pub const PRSPI: RegisterBits<Self> = RegisterBits::new(0x4);
    pub const PRSPI0: RegisterBits<Self> = RegisterBits::new(1<<2);

    pub const PRUSART0: RegisterBits<Self> = RegisterBits::new(0x2);
    pub const PRUSART00: RegisterBits<Self> = RegisterBits::new(1<<1);

    pub const PRADC: RegisterBits<Self> = RegisterBits::new(0x1);
    pub const PRADC0: RegisterBits<Self> = RegisterBits::new(1<<0);

}

impl Register for PRR0 {
    type T = u8;
    const ADDRESS: *mut u8 = 0x64 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct UESTA1X;

impl UESTA1X {
    pub const CTRLDIR: RegisterBits<Self> = RegisterBits::new(0x4);
    pub const CTRLDIR0: RegisterBits<Self> = RegisterBits::new(1<<2);

    pub const CURRBK: RegisterBits<Self> = RegisterBits::new(0x3);
    pub const CURRBK0: RegisterBits<Self> = RegisterBits::new(1<<0);
    pub const CURRBK1: RegisterBits<Self> = RegisterBits::new(1<<1);

}

impl Register for UESTA1X {
    type T = u8;
    const ADDRESS: *mut u8 = 0xef as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct PORTB;

impl PORTB {
}

impl Register for PORTB {
    type T = u8;
    const ADDRESS: *mut u8 = 0x25 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct OCR1B;

impl OCR1B {
}

impl Register for OCR1B {
    type T = u16;
    const ADDRESS: *mut u16 = 0x8a as *mut u16;
}
#[allow(non_camel_case_types)]
pub struct UEBCHX;

impl UEBCHX {
}

impl Register for UEBCHX {
    type T = u8;
    const ADDRESS: *mut u8 = 0xf3 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct PORTE;

impl PORTE {
}

impl Register for PORTE {
    type T = u8;
    const ADDRESS: *mut u8 = 0x2e as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct TWAR;

impl TWAR {
    pub const TWA: RegisterBits<Self> = RegisterBits::new(0xfe);
    pub const TWA0: RegisterBits<Self> = RegisterBits::new(1<<1);
    pub const TWA1: RegisterBits<Self> = RegisterBits::new(1<<2);
    pub const TWA2: RegisterBits<Self> = RegisterBits::new(1<<3);
    pub const TWA3: RegisterBits<Self> = RegisterBits::new(1<<4);
    pub const TWA4: RegisterBits<Self> = RegisterBits::new(1<<5);
    pub const TWA5: RegisterBits<Self> = RegisterBits::new(1<<6);
    pub const TWA6: RegisterBits<Self> = RegisterBits::new(1<<7);

    pub const TWGCE: RegisterBits<Self> = RegisterBits::new(0x1);
    pub const TWGCE0: RegisterBits<Self> = RegisterBits::new(1<<0);

}

impl Register for TWAR {
    type T = u8;
    const ADDRESS: *mut u8 = 0xba as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct OCR3A;

impl OCR3A {
}

impl Register for OCR3A {
    type T = u16;
    const ADDRESS: *mut u16 = 0x98 as *mut u16;
}
#[allow(non_camel_case_types)]
pub struct UDINT;

impl UDINT {
    pub const UPRSMI: RegisterBits<Self> = RegisterBits::new(0x40);
    pub const UPRSMI0: RegisterBits<Self> = RegisterBits::new(1<<6);

    pub const EORSMI: RegisterBits<Self> = RegisterBits::new(0x20);
    pub const EORSMI0: RegisterBits<Self> = RegisterBits::new(1<<5);

    pub const WAKEUPI: RegisterBits<Self> = RegisterBits::new(0x10);
    pub const WAKEUPI0: RegisterBits<Self> = RegisterBits::new(1<<4);

    pub const EORSTI: RegisterBits<Self> = RegisterBits::new(0x8);
    pub const EORSTI0: RegisterBits<Self> = RegisterBits::new(1<<3);

    pub const SOFI: RegisterBits<Self> = RegisterBits::new(0x4);
    pub const SOFI0: RegisterBits<Self> = RegisterBits::new(1<<2);

    pub const SUSPI: RegisterBits<Self> = RegisterBits::new(0x1);
    pub const SUSPI0: RegisterBits<Self> = RegisterBits::new(1<<0);

}

impl Register for UDINT {
    type T = u8;
    const ADDRESS: *mut u8 = 0xe1 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct TC4H;

impl TC4H {
}

impl Register for TC4H {
    type T = u8;
    const ADDRESS: *mut u8 = 0xbf as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct TCNT4;

impl TCNT4 {
}

impl Register for TCNT4 {
    type T = u8;
    const ADDRESS: *mut u8 = 0xbe as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct EICRB;

impl EICRB {
    pub const ISC7: RegisterBits<Self> = RegisterBits::new(0xc0);
    pub const ISC70: RegisterBits<Self> = RegisterBits::new(1<<6);
    pub const ISC71: RegisterBits<Self> = RegisterBits::new(1<<7);

    pub const ISC6: RegisterBits<Self> = RegisterBits::new(0x30);
    pub const ISC60: RegisterBits<Self> = RegisterBits::new(1<<4);
    pub const ISC61: RegisterBits<Self> = RegisterBits::new(1<<5);

    pub const ISC5: RegisterBits<Self> = RegisterBits::new(0xc);
    pub const ISC50: RegisterBits<Self> = RegisterBits::new(1<<2);
    pub const ISC51: RegisterBits<Self> = RegisterBits::new(1<<3);

    pub const ISC4: RegisterBits<Self> = RegisterBits::new(0x3);
    pub const ISC40: RegisterBits<Self> = RegisterBits::new(1<<0);
    pub const ISC41: RegisterBits<Self> = RegisterBits::new(1<<1);

}

impl Register for EICRB {
    type T = u8;
    const ADDRESS: *mut u8 = 0x6a as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct TCCR1A;

impl TCCR1A {
    pub const COM1A: RegisterBits<Self> = RegisterBits::new(0xc0);
    pub const COM1A0: RegisterBits<Self> = RegisterBits::new(1<<6);
    pub const COM1A1: RegisterBits<Self> = RegisterBits::new(1<<7);

    pub const COM1B: RegisterBits<Self> = RegisterBits::new(0x30);
    pub const COM1B0: RegisterBits<Self> = RegisterBits::new(1<<4);
    pub const COM1B1: RegisterBits<Self> = RegisterBits::new(1<<5);

    pub const COM1C: RegisterBits<Self> = RegisterBits::new(0xc);
    pub const COM1C0: RegisterBits<Self> = RegisterBits::new(1<<2);
    pub const COM1C1: RegisterBits<Self> = RegisterBits::new(1<<3);

    pub const WGM1: RegisterBits<Self> = RegisterBits::new(0x3);
    pub const WGM10: RegisterBits<Self> = RegisterBits::new(1<<0);
    pub const WGM11: RegisterBits<Self> = RegisterBits::new(1<<1);

}

impl Register for TCCR1A {
    type T = u8;
    const ADDRESS: *mut u8 = 0x80 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct TIMSK4;

impl TIMSK4 {
    pub const OCIE4D: RegisterBits<Self> = RegisterBits::new(0x80);
    pub const OCIE4D0: RegisterBits<Self> = RegisterBits::new(1<<7);

    pub const OCIE4A: RegisterBits<Self> = RegisterBits::new(0x40);
    pub const OCIE4A0: RegisterBits<Self> = RegisterBits::new(1<<6);

    pub const OCIE4B: RegisterBits<Self> = RegisterBits::new(0x20);
    pub const OCIE4B0: RegisterBits<Self> = RegisterBits::new(1<<5);

    pub const TOIE4: RegisterBits<Self> = RegisterBits::new(0x4);
    pub const TOIE40: RegisterBits<Self> = RegisterBits::new(1<<2);

}

impl Register for TIMSK4 {
    type T = u8;
    const ADDRESS: *mut u8 = 0x72 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct TIMSK1;

impl TIMSK1 {
    pub const ICIE1: RegisterBits<Self> = RegisterBits::new(0x20);
    pub const ICIE10: RegisterBits<Self> = RegisterBits::new(1<<5);

    pub const OCIE1C: RegisterBits<Self> = RegisterBits::new(0x8);
    pub const OCIE1C0: RegisterBits<Self> = RegisterBits::new(1<<3);

    pub const OCIE1B: RegisterBits<Self> = RegisterBits::new(0x4);
    pub const OCIE1B0: RegisterBits<Self> = RegisterBits::new(1<<2);

    pub const OCIE1A: RegisterBits<Self> = RegisterBits::new(0x2);
    pub const OCIE1A0: RegisterBits<Self> = RegisterBits::new(1<<1);

    pub const TOIE1: RegisterBits<Self> = RegisterBits::new(0x1);
    pub const TOIE10: RegisterBits<Self> = RegisterBits::new(1<<0);

}

impl Register for TIMSK1 {
    type T = u8;
    const ADDRESS: *mut u8 = 0x6f as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct ADCSRA;

impl ADCSRA {
    pub const ADEN: RegisterBits<Self> = RegisterBits::new(0x80);
    pub const ADEN0: RegisterBits<Self> = RegisterBits::new(1<<7);

    pub const ADSC: RegisterBits<Self> = RegisterBits::new(0x40);
    pub const ADSC0: RegisterBits<Self> = RegisterBits::new(1<<6);

    pub const ADATE: RegisterBits<Self> = RegisterBits::new(0x20);
    pub const ADATE0: RegisterBits<Self> = RegisterBits::new(1<<5);

    pub const ADIF: RegisterBits<Self> = RegisterBits::new(0x10);
    pub const ADIF0: RegisterBits<Self> = RegisterBits::new(1<<4);

    pub const ADIE: RegisterBits<Self> = RegisterBits::new(0x8);
    pub const ADIE0: RegisterBits<Self> = RegisterBits::new(1<<3);

    pub const ADPS: RegisterBits<Self> = RegisterBits::new(0x7);
    pub const ADPS0: RegisterBits<Self> = RegisterBits::new(1<<0);
    pub const ADPS1: RegisterBits<Self> = RegisterBits::new(1<<1);
    pub const ADPS2: RegisterBits<Self> = RegisterBits::new(1<<2);

}

impl Register for ADCSRA {
    type T = u8;
    const ADDRESS: *mut u8 = 0x7a as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct USBINT;

impl USBINT {
    pub const VBUSTI: RegisterBits<Self> = RegisterBits::new(0x1);
    pub const VBUSTI0: RegisterBits<Self> = RegisterBits::new(1<<0);

}

impl Register for USBINT {
    type T = u8;
    const ADDRESS: *mut u8 = 0xda as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct DIDR0;

impl DIDR0 {
    pub const ADC7D: RegisterBits<Self> = RegisterBits::new(0x80);
    pub const ADC7D0: RegisterBits<Self> = RegisterBits::new(1<<7);

    pub const ADC6D: RegisterBits<Self> = RegisterBits::new(0x40);
    pub const ADC6D0: RegisterBits<Self> = RegisterBits::new(1<<6);

    pub const ADC5D: RegisterBits<Self> = RegisterBits::new(0x20);
    pub const ADC5D0: RegisterBits<Self> = RegisterBits::new(1<<5);

    pub const ADC4D: RegisterBits<Self> = RegisterBits::new(0x10);
    pub const ADC4D0: RegisterBits<Self> = RegisterBits::new(1<<4);

    pub const ADC3D: RegisterBits<Self> = RegisterBits::new(0x8);
    pub const ADC3D0: RegisterBits<Self> = RegisterBits::new(1<<3);

    pub const ADC2D: RegisterBits<Self> = RegisterBits::new(0x4);
    pub const ADC2D0: RegisterBits<Self> = RegisterBits::new(1<<2);

    pub const ADC1D: RegisterBits<Self> = RegisterBits::new(0x2);
    pub const ADC1D0: RegisterBits<Self> = RegisterBits::new(1<<1);

    pub const ADC0D: RegisterBits<Self> = RegisterBits::new(0x1);
    pub const ADC0D0: RegisterBits<Self> = RegisterBits::new(1<<0);

}

impl Register for DIDR0 {
    type T = u8;
    const ADDRESS: *mut u8 = 0x7e as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct TWBR;

impl TWBR {
}

impl Register for TWBR {
    type T = u8;
    const ADDRESS: *mut u8 = 0xb8 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct UECONX;

impl UECONX {
    pub const STALLRQ: RegisterBits<Self> = RegisterBits::new(0x20);
    pub const STALLRQ0: RegisterBits<Self> = RegisterBits::new(1<<5);

    pub const STALLRQC: RegisterBits<Self> = RegisterBits::new(0x10);
    pub const STALLRQC0: RegisterBits<Self> = RegisterBits::new(1<<4);

    pub const RSTDT: RegisterBits<Self> = RegisterBits::new(0x8);
    pub const RSTDT0: RegisterBits<Self> = RegisterBits::new(1<<3);

    pub const EPEN: RegisterBits<Self> = RegisterBits::new(0x1);
    pub const EPEN0: RegisterBits<Self> = RegisterBits::new(1<<0);

}

impl Register for UECONX {
    type T = u8;
    const ADDRESS: *mut u8 = 0xeb as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct EIMSK;

impl EIMSK {
    pub const INT: RegisterBits<Self> = RegisterBits::new(0xff);
    pub const INT0: RegisterBits<Self> = RegisterBits::new(1<<0);
    pub const INT1: RegisterBits<Self> = RegisterBits::new(1<<1);
    pub const INT2: RegisterBits<Self> = RegisterBits::new(1<<2);
    pub const INT3: RegisterBits<Self> = RegisterBits::new(1<<3);
    pub const INT4: RegisterBits<Self> = RegisterBits::new(1<<4);
    pub const INT5: RegisterBits<Self> = RegisterBits::new(1<<5);
    pub const INT6: RegisterBits<Self> = RegisterBits::new(1<<6);
    pub const INT7: RegisterBits<Self> = RegisterBits::new(1<<7);

}

impl Register for EIMSK {
    type T = u8;
    const ADDRESS: *mut u8 = 0x3d as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct PCICR;

impl PCICR {
    pub const PCIE0: RegisterBits<Self> = RegisterBits::new(0x1);
    pub const PCIE00: RegisterBits<Self> = RegisterBits::new(1<<0);

}

impl Register for PCICR {
    type T = u8;
    const ADDRESS: *mut u8 = 0x68 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct GPIOR2;

impl GPIOR2 {
    pub const GPIOR: RegisterBits<Self> = RegisterBits::new(0xff);
    pub const GPIOR0: RegisterBits<Self> = RegisterBits::new(1<<0);
    pub const GPIOR1: RegisterBits<Self> = RegisterBits::new(1<<1);
    pub const GPIOR2: RegisterBits<Self> = RegisterBits::new(1<<2);
    pub const GPIOR3: RegisterBits<Self> = RegisterBits::new(1<<3);
    pub const GPIOR4: RegisterBits<Self> = RegisterBits::new(1<<4);
    pub const GPIOR5: RegisterBits<Self> = RegisterBits::new(1<<5);
    pub const GPIOR6: RegisterBits<Self> = RegisterBits::new(1<<6);
    pub const GPIOR7: RegisterBits<Self> = RegisterBits::new(1<<7);

}

impl Register for GPIOR2 {
    type T = u8;
    const ADDRESS: *mut u8 = 0x4b as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct TCCR0A;

impl TCCR0A {
    pub const COM0A: RegisterBits<Self> = RegisterBits::new(0xc0);
    pub const COM0A0: RegisterBits<Self> = RegisterBits::new(1<<6);
    pub const COM0A1: RegisterBits<Self> = RegisterBits::new(1<<7);

    pub const COM0B: RegisterBits<Self> = RegisterBits::new(0x30);
    pub const COM0B0: RegisterBits<Self> = RegisterBits::new(1<<4);
    pub const COM0B1: RegisterBits<Self> = RegisterBits::new(1<<5);

    pub const WGM0: RegisterBits<Self> = RegisterBits::new(0x3);
    pub const WGM00: RegisterBits<Self> = RegisterBits::new(1<<0);
    pub const WGM01: RegisterBits<Self> = RegisterBits::new(1<<1);

}

impl Register for TCCR0A {
    type T = u8;
    const ADDRESS: *mut u8 = 0x44 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct SPCR;

impl SPCR {
    pub const SPIE: RegisterBits<Self> = RegisterBits::new(0x80);
    pub const SPIE0: RegisterBits<Self> = RegisterBits::new(1<<7);

    pub const SPE: RegisterBits<Self> = RegisterBits::new(0x40);
    pub const SPE0: RegisterBits<Self> = RegisterBits::new(1<<6);

    pub const DORD: RegisterBits<Self> = RegisterBits::new(0x20);
    pub const DORD0: RegisterBits<Self> = RegisterBits::new(1<<5);

    pub const MSTR: RegisterBits<Self> = RegisterBits::new(0x10);
    pub const MSTR0: RegisterBits<Self> = RegisterBits::new(1<<4);

    pub const CPOL: RegisterBits<Self> = RegisterBits::new(0x8);
    pub const CPOL0: RegisterBits<Self> = RegisterBits::new(1<<3);

    pub const CPHA: RegisterBits<Self> = RegisterBits::new(0x4);
    pub const CPHA0: RegisterBits<Self> = RegisterBits::new(1<<2);

    pub const SPR: RegisterBits<Self> = RegisterBits::new(0x3);
    pub const SPR0: RegisterBits<Self> = RegisterBits::new(1<<0);
    pub const SPR1: RegisterBits<Self> = RegisterBits::new(1<<1);

}

impl Register for SPCR {
    type T = u8;
    const ADDRESS: *mut u8 = 0x4c as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct TCCR1C;

impl TCCR1C {
    pub const FOC1A: RegisterBits<Self> = RegisterBits::new(0x80);
    pub const FOC1A0: RegisterBits<Self> = RegisterBits::new(1<<7);

    pub const FOC1B: RegisterBits<Self> = RegisterBits::new(0x40);
    pub const FOC1B0: RegisterBits<Self> = RegisterBits::new(1<<6);

    pub const FOC1C: RegisterBits<Self> = RegisterBits::new(0x20);
    pub const FOC1C0: RegisterBits<Self> = RegisterBits::new(1<<5);

}

impl Register for TCCR1C {
    type T = u8;
    const ADDRESS: *mut u8 = 0x82 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct DIDR1;

impl DIDR1 {
    pub const AIN1D: RegisterBits<Self> = RegisterBits::new(0x2);
    pub const AIN1D0: RegisterBits<Self> = RegisterBits::new(1<<1);

    pub const AIN0D: RegisterBits<Self> = RegisterBits::new(0x1);
    pub const AIN0D0: RegisterBits<Self> = RegisterBits::new(1<<0);

}

impl Register for DIDR1 {
    type T = u8;
    const ADDRESS: *mut u8 = 0x7f as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct HIGH;

impl HIGH {
    pub const OCDEN: RegisterBits<Self> = RegisterBits::new(0x80);
    pub const OCDEN0: RegisterBits<Self> = RegisterBits::new(1<<7);

    pub const JTAGEN: RegisterBits<Self> = RegisterBits::new(0x40);
    pub const JTAGEN0: RegisterBits<Self> = RegisterBits::new(1<<6);

    pub const SPIEN: RegisterBits<Self> = RegisterBits::new(0x20);
    pub const SPIEN0: RegisterBits<Self> = RegisterBits::new(1<<5);

    pub const WDTON: RegisterBits<Self> = RegisterBits::new(0x10);
    pub const WDTON0: RegisterBits<Self> = RegisterBits::new(1<<4);

    pub const EESAVE: RegisterBits<Self> = RegisterBits::new(0x8);
    pub const EESAVE0: RegisterBits<Self> = RegisterBits::new(1<<3);

    pub const BOOTSZ: RegisterBits<Self> = RegisterBits::new(0x6);
    pub const BOOTSZ0: RegisterBits<Self> = RegisterBits::new(1<<1);
    pub const BOOTSZ1: RegisterBits<Self> = RegisterBits::new(1<<2);

    pub const BOOTRST: RegisterBits<Self> = RegisterBits::new(0x1);
    pub const BOOTRST0: RegisterBits<Self> = RegisterBits::new(1<<0);

}

impl Register for HIGH {
    type T = u8;
    const ADDRESS: *mut u8 = 0x1 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct UEDATX;

impl UEDATX {
    pub const DAT: RegisterBits<Self> = RegisterBits::new(0xff);
    pub const DAT0: RegisterBits<Self> = RegisterBits::new(1<<0);
    pub const DAT1: RegisterBits<Self> = RegisterBits::new(1<<1);
    pub const DAT2: RegisterBits<Self> = RegisterBits::new(1<<2);
    pub const DAT3: RegisterBits<Self> = RegisterBits::new(1<<3);
    pub const DAT4: RegisterBits<Self> = RegisterBits::new(1<<4);
    pub const DAT5: RegisterBits<Self> = RegisterBits::new(1<<5);
    pub const DAT6: RegisterBits<Self> = RegisterBits::new(1<<6);
    pub const DAT7: RegisterBits<Self> = RegisterBits::new(1<<7);

}

impl Register for UEDATX {
    type T = u8;
    const ADDRESS: *mut u8 = 0xf1 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct ADCSRB;

impl ADCSRB {
    pub const ADHSM: RegisterBits<Self> = RegisterBits::new(0x80);
    pub const ADHSM0: RegisterBits<Self> = RegisterBits::new(1<<7);

    pub const MUX5: RegisterBits<Self> = RegisterBits::new(0x20);
    pub const MUX50: RegisterBits<Self> = RegisterBits::new(1<<5);

    pub const ADTS: RegisterBits<Self> = RegisterBits::new(0x17);
    pub const ADTS0: RegisterBits<Self> = RegisterBits::new(1<<0);
    pub const ADTS1: RegisterBits<Self> = RegisterBits::new(1<<1);
    pub const ADTS2: RegisterBits<Self> = RegisterBits::new(1<<2);
    pub const ADTS3: RegisterBits<Self> = RegisterBits::new(1<<4);

    pub const ACME: RegisterBits<Self> = RegisterBits::new(0x40);
    pub const ACME0: RegisterBits<Self> = RegisterBits::new(1<<6);

}

impl Register for ADCSRB {
    type T = u8;
    const ADDRESS: *mut u8 = 0x7b as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct UHWCON;

impl UHWCON {
    pub const UVREGE: RegisterBits<Self> = RegisterBits::new(0x1);
    pub const UVREGE0: RegisterBits<Self> = RegisterBits::new(1<<0);

}

impl Register for UHWCON {
    type T = u8;
    const ADDRESS: *mut u8 = 0xd7 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct OCR0B;

impl OCR0B {
}

impl Register for OCR0B {
    type T = u8;
    const ADDRESS: *mut u8 = 0x48 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct OCR4D;

impl OCR4D {
}

impl Register for OCR4D {
    type T = u8;
    const ADDRESS: *mut u8 = 0xd2 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct MCUCR;

impl MCUCR {
    pub const JTD: RegisterBits<Self> = RegisterBits::new(0x80);
    pub const JTD0: RegisterBits<Self> = RegisterBits::new(1<<7);

    pub const PUD: RegisterBits<Self> = RegisterBits::new(0x10);
    pub const PUD0: RegisterBits<Self> = RegisterBits::new(1<<4);

    pub const IVSEL: RegisterBits<Self> = RegisterBits::new(0x2);
    pub const IVSEL0: RegisterBits<Self> = RegisterBits::new(1<<1);

    pub const IVCE: RegisterBits<Self> = RegisterBits::new(0x1);
    pub const IVCE0: RegisterBits<Self> = RegisterBits::new(1<<0);

}

impl Register for MCUCR {
    type T = u8;
    const ADDRESS: *mut u8 = 0x55 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct EIFR;

impl EIFR {
    pub const INTF: RegisterBits<Self> = RegisterBits::new(0xff);
    pub const INTF0: RegisterBits<Self> = RegisterBits::new(1<<0);
    pub const INTF1: RegisterBits<Self> = RegisterBits::new(1<<1);
    pub const INTF2: RegisterBits<Self> = RegisterBits::new(1<<2);
    pub const INTF3: RegisterBits<Self> = RegisterBits::new(1<<3);
    pub const INTF4: RegisterBits<Self> = RegisterBits::new(1<<4);
    pub const INTF5: RegisterBits<Self> = RegisterBits::new(1<<5);
    pub const INTF6: RegisterBits<Self> = RegisterBits::new(1<<6);
    pub const INTF7: RegisterBits<Self> = RegisterBits::new(1<<7);

}

impl Register for EIFR {
    type T = u8;
    const ADDRESS: *mut u8 = 0x3c as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct PLLCSR;

impl PLLCSR {
    pub const PINDIV: RegisterBits<Self> = RegisterBits::new(0x10);
    pub const PINDIV0: RegisterBits<Self> = RegisterBits::new(1<<4);

    pub const PLLE: RegisterBits<Self> = RegisterBits::new(0x2);
    pub const PLLE0: RegisterBits<Self> = RegisterBits::new(1<<1);

    pub const PLOCK: RegisterBits<Self> = RegisterBits::new(0x1);
    pub const PLOCK0: RegisterBits<Self> = RegisterBits::new(1<<0);

}

impl Register for PLLCSR {
    type T = u8;
    const ADDRESS: *mut u8 = 0x49 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct UBRR1;

impl UBRR1 {
}

impl Register for UBRR1 {
    type T = u16;
    const ADDRESS: *mut u16 = 0xcc as *mut u16;
}
#[allow(non_camel_case_types)]
pub struct PINC;

impl PINC {
}

impl Register for PINC {
    type T = u8;
    const ADDRESS: *mut u8 = 0x26 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct PCIFR;

impl PCIFR {
    pub const PCIF0: RegisterBits<Self> = RegisterBits::new(0x1);
    pub const PCIF00: RegisterBits<Self> = RegisterBits::new(1<<0);

}

impl Register for PCIFR {
    type T = u8;
    const ADDRESS: *mut u8 = 0x3b as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct TWCR;

impl TWCR {
    pub const TWINT: RegisterBits<Self> = RegisterBits::new(0x80);
    pub const TWINT0: RegisterBits<Self> = RegisterBits::new(1<<7);

    pub const TWEA: RegisterBits<Self> = RegisterBits::new(0x40);
    pub const TWEA0: RegisterBits<Self> = RegisterBits::new(1<<6);

    pub const TWSTA: RegisterBits<Self> = RegisterBits::new(0x20);
    pub const TWSTA0: RegisterBits<Self> = RegisterBits::new(1<<5);

    pub const TWSTO: RegisterBits<Self> = RegisterBits::new(0x10);
    pub const TWSTO0: RegisterBits<Self> = RegisterBits::new(1<<4);

    pub const TWWC: RegisterBits<Self> = RegisterBits::new(0x8);
    pub const TWWC0: RegisterBits<Self> = RegisterBits::new(1<<3);

    pub const TWEN: RegisterBits<Self> = RegisterBits::new(0x4);
    pub const TWEN0: RegisterBits<Self> = RegisterBits::new(1<<2);

    pub const TWIE: RegisterBits<Self> = RegisterBits::new(0x1);
    pub const TWIE0: RegisterBits<Self> = RegisterBits::new(1<<0);

}

impl Register for TWCR {
    type T = u8;
    const ADDRESS: *mut u8 = 0xbc as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct UEBCLX;

impl UEBCLX {
}

impl Register for UEBCLX {
    type T = u8;
    const ADDRESS: *mut u8 = 0xf2 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct DT4;

impl DT4 {
    pub const DT4L: RegisterBits<Self> = RegisterBits::new(0xff);
    pub const DT4L0: RegisterBits<Self> = RegisterBits::new(1<<0);
    pub const DT4L1: RegisterBits<Self> = RegisterBits::new(1<<1);
    pub const DT4L2: RegisterBits<Self> = RegisterBits::new(1<<2);
    pub const DT4L3: RegisterBits<Self> = RegisterBits::new(1<<3);
    pub const DT4L4: RegisterBits<Self> = RegisterBits::new(1<<4);
    pub const DT4L5: RegisterBits<Self> = RegisterBits::new(1<<5);
    pub const DT4L6: RegisterBits<Self> = RegisterBits::new(1<<6);
    pub const DT4L7: RegisterBits<Self> = RegisterBits::new(1<<7);

}

impl Register for DT4 {
    type T = u8;
    const ADDRESS: *mut u8 = 0xd4 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct UDR1;

impl UDR1 {
}

impl Register for UDR1 {
    type T = u8;
    const ADDRESS: *mut u8 = 0xce as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct ACSR;

impl ACSR {
    pub const ACD: RegisterBits<Self> = RegisterBits::new(0x80);
    pub const ACD0: RegisterBits<Self> = RegisterBits::new(1<<7);

    pub const ACBG: RegisterBits<Self> = RegisterBits::new(0x40);
    pub const ACBG0: RegisterBits<Self> = RegisterBits::new(1<<6);

    pub const ACO: RegisterBits<Self> = RegisterBits::new(0x20);
    pub const ACO0: RegisterBits<Self> = RegisterBits::new(1<<5);

    pub const ACI: RegisterBits<Self> = RegisterBits::new(0x10);
    pub const ACI0: RegisterBits<Self> = RegisterBits::new(1<<4);

    pub const ACIE: RegisterBits<Self> = RegisterBits::new(0x8);
    pub const ACIE0: RegisterBits<Self> = RegisterBits::new(1<<3);

    pub const ACIC: RegisterBits<Self> = RegisterBits::new(0x4);
    pub const ACIC0: RegisterBits<Self> = RegisterBits::new(1<<2);

    pub const ACIS: RegisterBits<Self> = RegisterBits::new(0x3);
    pub const ACIS0: RegisterBits<Self> = RegisterBits::new(1<<0);
    pub const ACIS1: RegisterBits<Self> = RegisterBits::new(1<<1);

}

impl Register for ACSR {
    type T = u8;
    const ADDRESS: *mut u8 = 0x50 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct UECFG0X;

impl UECFG0X {
    pub const EPTYPE: RegisterBits<Self> = RegisterBits::new(0xc0);
    pub const EPTYPE0: RegisterBits<Self> = RegisterBits::new(1<<6);
    pub const EPTYPE1: RegisterBits<Self> = RegisterBits::new(1<<7);

    pub const EPDIR: RegisterBits<Self> = RegisterBits::new(0x1);
    pub const EPDIR0: RegisterBits<Self> = RegisterBits::new(1<<0);

}

impl Register for UECFG0X {
    type T = u8;
    const ADDRESS: *mut u8 = 0xec as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct TCNT3;

impl TCNT3 {
}

impl Register for TCNT3 {
    type T = u16;
    const ADDRESS: *mut u16 = 0x94 as *mut u16;
}
#[allow(non_camel_case_types)]
pub struct TIFR3;

impl TIFR3 {
    pub const ICF3: RegisterBits<Self> = RegisterBits::new(0x20);
    pub const ICF30: RegisterBits<Self> = RegisterBits::new(1<<5);

    pub const OCF3C: RegisterBits<Self> = RegisterBits::new(0x8);
    pub const OCF3C0: RegisterBits<Self> = RegisterBits::new(1<<3);

    pub const OCF3B: RegisterBits<Self> = RegisterBits::new(0x4);
    pub const OCF3B0: RegisterBits<Self> = RegisterBits::new(1<<2);

    pub const OCF3A: RegisterBits<Self> = RegisterBits::new(0x2);
    pub const OCF3A0: RegisterBits<Self> = RegisterBits::new(1<<1);

    pub const TOV3: RegisterBits<Self> = RegisterBits::new(0x1);
    pub const TOV30: RegisterBits<Self> = RegisterBits::new(1<<0);

}

impl Register for TIFR3 {
    type T = u8;
    const ADDRESS: *mut u8 = 0x38 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct TCCR0B;

impl TCCR0B {
    pub const FOC0A: RegisterBits<Self> = RegisterBits::new(0x80);
    pub const FOC0A0: RegisterBits<Self> = RegisterBits::new(1<<7);

    pub const FOC0B: RegisterBits<Self> = RegisterBits::new(0x40);
    pub const FOC0B0: RegisterBits<Self> = RegisterBits::new(1<<6);

    pub const WGM02: RegisterBits<Self> = RegisterBits::new(0x8);
    pub const WGM020: RegisterBits<Self> = RegisterBits::new(1<<3);

    pub const CS0: RegisterBits<Self> = RegisterBits::new(0x7);
    pub const CS00: RegisterBits<Self> = RegisterBits::new(1<<0);
    pub const CS01: RegisterBits<Self> = RegisterBits::new(1<<1);
    pub const CS02: RegisterBits<Self> = RegisterBits::new(1<<2);

}

impl Register for TCCR0B {
    type T = u8;
    const ADDRESS: *mut u8 = 0x45 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct UEINT;

impl UEINT {
}

impl Register for UEINT {
    type T = u8;
    const ADDRESS: *mut u8 = 0xf4 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct PINF;

impl PINF {
}

impl Register for PINF {
    type T = u8;
    const ADDRESS: *mut u8 = 0x2f as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct TIFR4;

impl TIFR4 {
    pub const OCF4D: RegisterBits<Self> = RegisterBits::new(0x80);
    pub const OCF4D0: RegisterBits<Self> = RegisterBits::new(1<<7);

    pub const OCF4A: RegisterBits<Self> = RegisterBits::new(0x40);
    pub const OCF4A0: RegisterBits<Self> = RegisterBits::new(1<<6);

    pub const OCF4B: RegisterBits<Self> = RegisterBits::new(0x20);
    pub const OCF4B0: RegisterBits<Self> = RegisterBits::new(1<<5);

    pub const TOV4: RegisterBits<Self> = RegisterBits::new(0x4);
    pub const TOV40: RegisterBits<Self> = RegisterBits::new(1<<2);

}

impl Register for TIFR4 {
    type T = u8;
    const ADDRESS: *mut u8 = 0x39 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct UDADDR;

impl UDADDR {
    pub const ADDEN: RegisterBits<Self> = RegisterBits::new(0x80);
    pub const ADDEN0: RegisterBits<Self> = RegisterBits::new(1<<7);

    pub const UADD: RegisterBits<Self> = RegisterBits::new(0x7f);
    pub const UADD0: RegisterBits<Self> = RegisterBits::new(1<<0);
    pub const UADD1: RegisterBits<Self> = RegisterBits::new(1<<1);
    pub const UADD2: RegisterBits<Self> = RegisterBits::new(1<<2);
    pub const UADD3: RegisterBits<Self> = RegisterBits::new(1<<3);
    pub const UADD4: RegisterBits<Self> = RegisterBits::new(1<<4);
    pub const UADD5: RegisterBits<Self> = RegisterBits::new(1<<5);
    pub const UADD6: RegisterBits<Self> = RegisterBits::new(1<<6);

}

impl Register for UDADDR {
    type T = u8;
    const ADDRESS: *mut u8 = 0xe3 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct UDMFN;

impl UDMFN {
    pub const FNCERR: RegisterBits<Self> = RegisterBits::new(0x10);
    pub const FNCERR0: RegisterBits<Self> = RegisterBits::new(1<<4);

}

impl Register for UDMFN {
    type T = u8;
    const ADDRESS: *mut u8 = 0xe6 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct DDRF;

impl DDRF {
}

impl Register for DDRF {
    type T = u8;
    const ADDRESS: *mut u8 = 0x30 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct UERST;

impl UERST {
    pub const EPRST: RegisterBits<Self> = RegisterBits::new(0x7f);
    pub const EPRST0: RegisterBits<Self> = RegisterBits::new(1<<0);
    pub const EPRST1: RegisterBits<Self> = RegisterBits::new(1<<1);
    pub const EPRST2: RegisterBits<Self> = RegisterBits::new(1<<2);
    pub const EPRST3: RegisterBits<Self> = RegisterBits::new(1<<3);
    pub const EPRST4: RegisterBits<Self> = RegisterBits::new(1<<4);
    pub const EPRST5: RegisterBits<Self> = RegisterBits::new(1<<5);
    pub const EPRST6: RegisterBits<Self> = RegisterBits::new(1<<6);

}

impl Register for UERST {
    type T = u8;
    const ADDRESS: *mut u8 = 0xea as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct OCR4A;

impl OCR4A {
}

impl Register for OCR4A {
    type T = u8;
    const ADDRESS: *mut u8 = 0xcf as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct ICR3;

impl ICR3 {
}

impl Register for ICR3 {
    type T = u16;
    const ADDRESS: *mut u16 = 0x96 as *mut u16;
}
#[allow(non_camel_case_types)]
pub struct SMCR;

impl SMCR {
    pub const SM: RegisterBits<Self> = RegisterBits::new(0xe);
    pub const SM0: RegisterBits<Self> = RegisterBits::new(1<<1);
    pub const SM1: RegisterBits<Self> = RegisterBits::new(1<<2);
    pub const SM2: RegisterBits<Self> = RegisterBits::new(1<<3);

    pub const SE: RegisterBits<Self> = RegisterBits::new(0x1);
    pub const SE0: RegisterBits<Self> = RegisterBits::new(1<<0);

}

impl Register for SMCR {
    type T = u8;
    const ADDRESS: *mut u8 = 0x53 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct UEINTX;

impl UEINTX {
    pub const FIFOCON: RegisterBits<Self> = RegisterBits::new(0x80);
    pub const FIFOCON0: RegisterBits<Self> = RegisterBits::new(1<<7);

    pub const NAKINI: RegisterBits<Self> = RegisterBits::new(0x40);
    pub const NAKINI0: RegisterBits<Self> = RegisterBits::new(1<<6);

    pub const RWAL: RegisterBits<Self> = RegisterBits::new(0x20);
    pub const RWAL0: RegisterBits<Self> = RegisterBits::new(1<<5);

    pub const NAKOUTI: RegisterBits<Self> = RegisterBits::new(0x10);
    pub const NAKOUTI0: RegisterBits<Self> = RegisterBits::new(1<<4);

    pub const RXSTPI: RegisterBits<Self> = RegisterBits::new(0x8);
    pub const RXSTPI0: RegisterBits<Self> = RegisterBits::new(1<<3);

    pub const RXOUTI: RegisterBits<Self> = RegisterBits::new(0x4);
    pub const RXOUTI0: RegisterBits<Self> = RegisterBits::new(1<<2);

    pub const STALLEDI: RegisterBits<Self> = RegisterBits::new(0x2);
    pub const STALLEDI0: RegisterBits<Self> = RegisterBits::new(1<<1);

    pub const TXINI: RegisterBits<Self> = RegisterBits::new(0x1);
    pub const TXINI0: RegisterBits<Self> = RegisterBits::new(1<<0);

}

impl Register for UEINTX {
    type T = u8;
    const ADDRESS: *mut u8 = 0xe8 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct LOW;

impl LOW {
    pub const CKDIV8: RegisterBits<Self> = RegisterBits::new(0x80);
    pub const CKDIV80: RegisterBits<Self> = RegisterBits::new(1<<7);

    pub const CKOUT: RegisterBits<Self> = RegisterBits::new(0x40);
    pub const CKOUT0: RegisterBits<Self> = RegisterBits::new(1<<6);

}

impl Register for LOW {
    type T = u8;
    const ADDRESS: *mut u8 = 0x0 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct PRR1;

impl PRR1 {
    pub const PRUSB: RegisterBits<Self> = RegisterBits::new(0x80);
    pub const PRUSB0: RegisterBits<Self> = RegisterBits::new(1<<7);

    pub const PRTIM4: RegisterBits<Self> = RegisterBits::new(0x10);
    pub const PRTIM40: RegisterBits<Self> = RegisterBits::new(1<<4);

    pub const PRTIM3: RegisterBits<Self> = RegisterBits::new(0x8);
    pub const PRTIM30: RegisterBits<Self> = RegisterBits::new(1<<3);

    pub const PRUSART1: RegisterBits<Self> = RegisterBits::new(0x1);
    pub const PRUSART10: RegisterBits<Self> = RegisterBits::new(1<<0);

}

impl Register for PRR1 {
    type T = u8;
    const ADDRESS: *mut u8 = 0x65 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct GPIOR1;

impl GPIOR1 {
    pub const GPIOR: RegisterBits<Self> = RegisterBits::new(0xff);
    pub const GPIOR0: RegisterBits<Self> = RegisterBits::new(1<<0);
    pub const GPIOR1: RegisterBits<Self> = RegisterBits::new(1<<1);
    pub const GPIOR2: RegisterBits<Self> = RegisterBits::new(1<<2);
    pub const GPIOR3: RegisterBits<Self> = RegisterBits::new(1<<3);
    pub const GPIOR4: RegisterBits<Self> = RegisterBits::new(1<<4);
    pub const GPIOR5: RegisterBits<Self> = RegisterBits::new(1<<5);
    pub const GPIOR6: RegisterBits<Self> = RegisterBits::new(1<<6);
    pub const GPIOR7: RegisterBits<Self> = RegisterBits::new(1<<7);

}

impl Register for GPIOR1 {
    type T = u8;
    const ADDRESS: *mut u8 = 0x4a as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct TCCR4E;

impl TCCR4E {
    pub const TLOCK4: RegisterBits<Self> = RegisterBits::new(0x80);
    pub const TLOCK40: RegisterBits<Self> = RegisterBits::new(1<<7);

    pub const ENHC4: RegisterBits<Self> = RegisterBits::new(0x40);
    pub const ENHC40: RegisterBits<Self> = RegisterBits::new(1<<6);

    pub const OC4OE: RegisterBits<Self> = RegisterBits::new(0x3f);
    pub const OC4OE0: RegisterBits<Self> = RegisterBits::new(1<<0);
    pub const OC4OE1: RegisterBits<Self> = RegisterBits::new(1<<1);
    pub const OC4OE2: RegisterBits<Self> = RegisterBits::new(1<<2);
    pub const OC4OE3: RegisterBits<Self> = RegisterBits::new(1<<3);
    pub const OC4OE4: RegisterBits<Self> = RegisterBits::new(1<<4);
    pub const OC4OE5: RegisterBits<Self> = RegisterBits::new(1<<5);

}

impl Register for TCCR4E {
    type T = u8;
    const ADDRESS: *mut u8 = 0xc4 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct PINE;

impl PINE {
}

impl Register for PINE {
    type T = u8;
    const ADDRESS: *mut u8 = 0x2c as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct EECR;

impl EECR {
    pub const EEPM: RegisterBits<Self> = RegisterBits::new(0x30);
    pub const EEPM0: RegisterBits<Self> = RegisterBits::new(1<<4);
    pub const EEPM1: RegisterBits<Self> = RegisterBits::new(1<<5);

    pub const EERIE: RegisterBits<Self> = RegisterBits::new(0x8);
    pub const EERIE0: RegisterBits<Self> = RegisterBits::new(1<<3);

    pub const EEMPE: RegisterBits<Self> = RegisterBits::new(0x4);
    pub const EEMPE0: RegisterBits<Self> = RegisterBits::new(1<<2);

    pub const EEPE: RegisterBits<Self> = RegisterBits::new(0x2);
    pub const EEPE0: RegisterBits<Self> = RegisterBits::new(1<<1);

    pub const EERE: RegisterBits<Self> = RegisterBits::new(0x1);
    pub const EERE0: RegisterBits<Self> = RegisterBits::new(1<<0);

}

impl Register for EECR {
    type T = u8;
    const ADDRESS: *mut u8 = 0x3f as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct RCCTRL;

impl RCCTRL {
    pub const RCFREQ: RegisterBits<Self> = RegisterBits::new(0x1);
    pub const RCFREQ0: RegisterBits<Self> = RegisterBits::new(1<<0);

}

impl Register for RCCTRL {
    type T = u8;
    const ADDRESS: *mut u8 = 0x67 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct PINB;

impl PINB {
}

impl Register for PINB {
    type T = u8;
    const ADDRESS: *mut u8 = 0x23 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct SPMCSR;

impl SPMCSR {
    pub const SPMIE: RegisterBits<Self> = RegisterBits::new(0x80);
    pub const SPMIE0: RegisterBits<Self> = RegisterBits::new(1<<7);

    pub const RWWSB: RegisterBits<Self> = RegisterBits::new(0x40);
    pub const RWWSB0: RegisterBits<Self> = RegisterBits::new(1<<6);

    pub const SIGRD: RegisterBits<Self> = RegisterBits::new(0x20);
    pub const SIGRD0: RegisterBits<Self> = RegisterBits::new(1<<5);

    pub const RWWSRE: RegisterBits<Self> = RegisterBits::new(0x10);
    pub const RWWSRE0: RegisterBits<Self> = RegisterBits::new(1<<4);

    pub const BLBSET: RegisterBits<Self> = RegisterBits::new(0x8);
    pub const BLBSET0: RegisterBits<Self> = RegisterBits::new(1<<3);

    pub const PGWRT: RegisterBits<Self> = RegisterBits::new(0x4);
    pub const PGWRT0: RegisterBits<Self> = RegisterBits::new(1<<2);

    pub const PGERS: RegisterBits<Self> = RegisterBits::new(0x2);
    pub const PGERS0: RegisterBits<Self> = RegisterBits::new(1<<1);

    pub const SPMEN: RegisterBits<Self> = RegisterBits::new(0x1);
    pub const SPMEN0: RegisterBits<Self> = RegisterBits::new(1<<0);

}

impl Register for SPMCSR {
    type T = u8;
    const ADDRESS: *mut u8 = 0x57 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct UCSR1B;

impl UCSR1B {
    pub const RXCIE1: RegisterBits<Self> = RegisterBits::new(0x80);
    pub const RXCIE10: RegisterBits<Self> = RegisterBits::new(1<<7);

    pub const TXCIE1: RegisterBits<Self> = RegisterBits::new(0x40);
    pub const TXCIE10: RegisterBits<Self> = RegisterBits::new(1<<6);

    pub const UDRIE1: RegisterBits<Self> = RegisterBits::new(0x20);
    pub const UDRIE10: RegisterBits<Self> = RegisterBits::new(1<<5);

    pub const RXEN1: RegisterBits<Self> = RegisterBits::new(0x10);
    pub const RXEN10: RegisterBits<Self> = RegisterBits::new(1<<4);

    pub const TXEN1: RegisterBits<Self> = RegisterBits::new(0x8);
    pub const TXEN10: RegisterBits<Self> = RegisterBits::new(1<<3);

    pub const UCSZ12: RegisterBits<Self> = RegisterBits::new(0x4);
    pub const UCSZ120: RegisterBits<Self> = RegisterBits::new(1<<2);

    pub const RXB81: RegisterBits<Self> = RegisterBits::new(0x2);
    pub const RXB810: RegisterBits<Self> = RegisterBits::new(1<<1);

    pub const TXB81: RegisterBits<Self> = RegisterBits::new(0x1);
    pub const TXB810: RegisterBits<Self> = RegisterBits::new(1<<0);

}

impl Register for UCSR1B {
    type T = u8;
    const ADDRESS: *mut u8 = 0xc9 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct OCR4B;

impl OCR4B {
}

impl Register for OCR4B {
    type T = u8;
    const ADDRESS: *mut u8 = 0xd0 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct PLLFRQ;

impl PLLFRQ {
    pub const PINMUX: RegisterBits<Self> = RegisterBits::new(0x80);
    pub const PINMUX0: RegisterBits<Self> = RegisterBits::new(1<<7);

    pub const PLLUSB: RegisterBits<Self> = RegisterBits::new(0x40);
    pub const PLLUSB0: RegisterBits<Self> = RegisterBits::new(1<<6);

    pub const PLLTM: RegisterBits<Self> = RegisterBits::new(0x30);
    pub const PLLTM0: RegisterBits<Self> = RegisterBits::new(1<<4);
    pub const PLLTM1: RegisterBits<Self> = RegisterBits::new(1<<5);

    pub const PDIV: RegisterBits<Self> = RegisterBits::new(0xf);
    pub const PDIV0: RegisterBits<Self> = RegisterBits::new(1<<0);
    pub const PDIV1: RegisterBits<Self> = RegisterBits::new(1<<1);
    pub const PDIV2: RegisterBits<Self> = RegisterBits::new(1<<2);
    pub const PDIV3: RegisterBits<Self> = RegisterBits::new(1<<3);

}

impl Register for PLLFRQ {
    type T = u8;
    const ADDRESS: *mut u8 = 0x52 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct CLKSEL0;

impl CLKSEL0 {
    pub const RCSUT: RegisterBits<Self> = RegisterBits::new(0xc0);
    pub const RCSUT0: RegisterBits<Self> = RegisterBits::new(1<<6);
    pub const RCSUT1: RegisterBits<Self> = RegisterBits::new(1<<7);

    pub const EXSUT: RegisterBits<Self> = RegisterBits::new(0x30);
    pub const EXSUT0: RegisterBits<Self> = RegisterBits::new(1<<4);
    pub const EXSUT1: RegisterBits<Self> = RegisterBits::new(1<<5);

    pub const RCE: RegisterBits<Self> = RegisterBits::new(0x8);
    pub const RCE0: RegisterBits<Self> = RegisterBits::new(1<<3);

    pub const EXTE: RegisterBits<Self> = RegisterBits::new(0x4);
    pub const EXTE0: RegisterBits<Self> = RegisterBits::new(1<<2);

    pub const CLKS: RegisterBits<Self> = RegisterBits::new(0x1);
    pub const CLKS0: RegisterBits<Self> = RegisterBits::new(1<<0);

}

impl Register for CLKSEL0 {
    type T = u8;
    const ADDRESS: *mut u8 = 0xc5 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct OCR1A;

impl OCR1A {
}

impl Register for OCR1A {
    type T = u16;
    const ADDRESS: *mut u16 = 0x88 as *mut u16;
}
#[allow(non_camel_case_types)]
pub struct PORTD;

impl PORTD {
}

impl Register for PORTD {
    type T = u8;
    const ADDRESS: *mut u8 = 0x2b as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct UEIENX;

impl UEIENX {
    pub const FLERRE: RegisterBits<Self> = RegisterBits::new(0x80);
    pub const FLERRE0: RegisterBits<Self> = RegisterBits::new(1<<7);

    pub const NAKINE: RegisterBits<Self> = RegisterBits::new(0x40);
    pub const NAKINE0: RegisterBits<Self> = RegisterBits::new(1<<6);

    pub const NAKOUTE: RegisterBits<Self> = RegisterBits::new(0x10);
    pub const NAKOUTE0: RegisterBits<Self> = RegisterBits::new(1<<4);

    pub const RXSTPE: RegisterBits<Self> = RegisterBits::new(0x8);
    pub const RXSTPE0: RegisterBits<Self> = RegisterBits::new(1<<3);

    pub const RXOUTE: RegisterBits<Self> = RegisterBits::new(0x4);
    pub const RXOUTE0: RegisterBits<Self> = RegisterBits::new(1<<2);

    pub const STALLEDE: RegisterBits<Self> = RegisterBits::new(0x2);
    pub const STALLEDE0: RegisterBits<Self> = RegisterBits::new(1<<1);

    pub const TXINE: RegisterBits<Self> = RegisterBits::new(0x1);
    pub const TXINE0: RegisterBits<Self> = RegisterBits::new(1<<0);

}

impl Register for UEIENX {
    type T = u8;
    const ADDRESS: *mut u8 = 0xf0 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct PIND;

impl PIND {
}

impl Register for PIND {
    type T = u8;
    const ADDRESS: *mut u8 = 0x29 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct PORTC;

impl PORTC {
}

impl Register for PORTC {
    type T = u8;
    const ADDRESS: *mut u8 = 0x28 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct TIMSK0;

impl TIMSK0 {
    pub const OCIE0B: RegisterBits<Self> = RegisterBits::new(0x4);
    pub const OCIE0B0: RegisterBits<Self> = RegisterBits::new(1<<2);

    pub const OCIE0A: RegisterBits<Self> = RegisterBits::new(0x2);
    pub const OCIE0A0: RegisterBits<Self> = RegisterBits::new(1<<1);

    pub const TOIE0: RegisterBits<Self> = RegisterBits::new(0x1);
    pub const TOIE00: RegisterBits<Self> = RegisterBits::new(1<<0);

}

impl Register for TIMSK0 {
    type T = u8;
    const ADDRESS: *mut u8 = 0x6e as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct RAMPZ;

impl RAMPZ {
    pub const RAMPZ: RegisterBits<Self> = RegisterBits::new(0x3);
    pub const RAMPZ0: RegisterBits<Self> = RegisterBits::new(1<<0);
    pub const RAMPZ1: RegisterBits<Self> = RegisterBits::new(1<<1);

}

impl Register for RAMPZ {
    type T = u8;
    const ADDRESS: *mut u8 = 0x5b as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct DDRD;

impl DDRD {
}

impl Register for DDRD {
    type T = u8;
    const ADDRESS: *mut u8 = 0x2a as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct UCSR1A;

impl UCSR1A {
    pub const RXC1: RegisterBits<Self> = RegisterBits::new(0x80);
    pub const RXC10: RegisterBits<Self> = RegisterBits::new(1<<7);

    pub const TXC1: RegisterBits<Self> = RegisterBits::new(0x40);
    pub const TXC10: RegisterBits<Self> = RegisterBits::new(1<<6);

    pub const UDRE1: RegisterBits<Self> = RegisterBits::new(0x20);
    pub const UDRE10: RegisterBits<Self> = RegisterBits::new(1<<5);

    pub const FE1: RegisterBits<Self> = RegisterBits::new(0x10);
    pub const FE10: RegisterBits<Self> = RegisterBits::new(1<<4);

    pub const DOR1: RegisterBits<Self> = RegisterBits::new(0x8);
    pub const DOR10: RegisterBits<Self> = RegisterBits::new(1<<3);

    pub const UPE1: RegisterBits<Self> = RegisterBits::new(0x4);
    pub const UPE10: RegisterBits<Self> = RegisterBits::new(1<<2);

    pub const U2X1: RegisterBits<Self> = RegisterBits::new(0x2);
    pub const U2X10: RegisterBits<Self> = RegisterBits::new(1<<1);

    pub const MPCM1: RegisterBits<Self> = RegisterBits::new(0x1);
    pub const MPCM10: RegisterBits<Self> = RegisterBits::new(1<<0);

}

impl Register for UCSR1A {
    type T = u8;
    const ADDRESS: *mut u8 = 0xc8 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct UCSR1C;

impl UCSR1C {
    pub const UMSEL1: RegisterBits<Self> = RegisterBits::new(0xc0);
    pub const UMSEL10: RegisterBits<Self> = RegisterBits::new(1<<6);
    pub const UMSEL11: RegisterBits<Self> = RegisterBits::new(1<<7);

    pub const UPM1: RegisterBits<Self> = RegisterBits::new(0x30);
    pub const UPM10: RegisterBits<Self> = RegisterBits::new(1<<4);
    pub const UPM11: RegisterBits<Self> = RegisterBits::new(1<<5);

    pub const USBS1: RegisterBits<Self> = RegisterBits::new(0x8);
    pub const USBS10: RegisterBits<Self> = RegisterBits::new(1<<3);

    pub const UCSZ1: RegisterBits<Self> = RegisterBits::new(0x6);
    pub const UCSZ10: RegisterBits<Self> = RegisterBits::new(1<<1);
    pub const UCSZ11: RegisterBits<Self> = RegisterBits::new(1<<2);

    pub const UCPOL1: RegisterBits<Self> = RegisterBits::new(0x1);
    pub const UCPOL10: RegisterBits<Self> = RegisterBits::new(1<<0);

}

impl Register for UCSR1C {
    type T = u8;
    const ADDRESS: *mut u8 = 0xca as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct DDRC;

impl DDRC {
}

impl Register for DDRC {
    type T = u8;
    const ADDRESS: *mut u8 = 0x27 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct OCR3B;

impl OCR3B {
}

impl Register for OCR3B {
    type T = u16;
    const ADDRESS: *mut u16 = 0x9a as *mut u16;
}
#[allow(non_camel_case_types)]
pub struct EXTENDED;

impl EXTENDED {
    pub const BODLEVEL: RegisterBits<Self> = RegisterBits::new(0x7);
    pub const BODLEVEL0: RegisterBits<Self> = RegisterBits::new(1<<0);
    pub const BODLEVEL1: RegisterBits<Self> = RegisterBits::new(1<<1);
    pub const BODLEVEL2: RegisterBits<Self> = RegisterBits::new(1<<2);

    pub const HWBE: RegisterBits<Self> = RegisterBits::new(0x8);
    pub const HWBE0: RegisterBits<Self> = RegisterBits::new(1<<3);

}

impl Register for EXTENDED {
    type T = u8;
    const ADDRESS: *mut u8 = 0x2 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct ADMUX;

impl ADMUX {
    pub const REFS: RegisterBits<Self> = RegisterBits::new(0xc0);
    pub const REFS0: RegisterBits<Self> = RegisterBits::new(1<<6);
    pub const REFS1: RegisterBits<Self> = RegisterBits::new(1<<7);

    pub const ADLAR: RegisterBits<Self> = RegisterBits::new(0x20);
    pub const ADLAR0: RegisterBits<Self> = RegisterBits::new(1<<5);

    pub const MUX: RegisterBits<Self> = RegisterBits::new(0x1f);
    pub const MUX0: RegisterBits<Self> = RegisterBits::new(1<<0);
    pub const MUX1: RegisterBits<Self> = RegisterBits::new(1<<1);
    pub const MUX2: RegisterBits<Self> = RegisterBits::new(1<<2);
    pub const MUX3: RegisterBits<Self> = RegisterBits::new(1<<3);
    pub const MUX4: RegisterBits<Self> = RegisterBits::new(1<<4);

}

impl Register for ADMUX {
    type T = u8;
    const ADDRESS: *mut u8 = 0x7c as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct UENUM;

impl UENUM {
}

impl Register for UENUM {
    type T = u8;
    const ADDRESS: *mut u8 = 0xe9 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct UCSR1D;

impl UCSR1D {
    pub const CTSEN: RegisterBits<Self> = RegisterBits::new(0x2);
    pub const CTSEN0: RegisterBits<Self> = RegisterBits::new(1<<1);

    pub const RTSEN: RegisterBits<Self> = RegisterBits::new(0x1);
    pub const RTSEN0: RegisterBits<Self> = RegisterBits::new(1<<0);

}

impl Register for UCSR1D {
    type T = u8;
    const ADDRESS: *mut u8 = 0xcb as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct EICRA;

impl EICRA {
    pub const ISC3: RegisterBits<Self> = RegisterBits::new(0xc0);
    pub const ISC30: RegisterBits<Self> = RegisterBits::new(1<<6);
    pub const ISC31: RegisterBits<Self> = RegisterBits::new(1<<7);

    pub const ISC2: RegisterBits<Self> = RegisterBits::new(0x30);
    pub const ISC20: RegisterBits<Self> = RegisterBits::new(1<<4);
    pub const ISC21: RegisterBits<Self> = RegisterBits::new(1<<5);

    pub const ISC1: RegisterBits<Self> = RegisterBits::new(0xc);
    pub const ISC10: RegisterBits<Self> = RegisterBits::new(1<<2);
    pub const ISC11: RegisterBits<Self> = RegisterBits::new(1<<3);

    pub const ISC0: RegisterBits<Self> = RegisterBits::new(0x3);
    pub const ISC00: RegisterBits<Self> = RegisterBits::new(1<<0);
    pub const ISC01: RegisterBits<Self> = RegisterBits::new(1<<1);

}

impl Register for EICRA {
    type T = u8;
    const ADDRESS: *mut u8 = 0x69 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct USBSTA;

impl USBSTA {
    pub const SPEED: RegisterBits<Self> = RegisterBits::new(0x8);
    pub const SPEED0: RegisterBits<Self> = RegisterBits::new(1<<3);

    pub const VBUS: RegisterBits<Self> = RegisterBits::new(0x1);
    pub const VBUS0: RegisterBits<Self> = RegisterBits::new(1<<0);

}

impl Register for USBSTA {
    type T = u8;
    const ADDRESS: *mut u8 = 0xd9 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct WDTCSR;

impl WDTCSR {
    pub const WDIF: RegisterBits<Self> = RegisterBits::new(0x80);
    pub const WDIF0: RegisterBits<Self> = RegisterBits::new(1<<7);

    pub const WDIE: RegisterBits<Self> = RegisterBits::new(0x40);
    pub const WDIE0: RegisterBits<Self> = RegisterBits::new(1<<6);

    pub const WDP: RegisterBits<Self> = RegisterBits::new(0x27);
    pub const WDP0: RegisterBits<Self> = RegisterBits::new(1<<0);
    pub const WDP1: RegisterBits<Self> = RegisterBits::new(1<<1);
    pub const WDP2: RegisterBits<Self> = RegisterBits::new(1<<2);
    pub const WDP3: RegisterBits<Self> = RegisterBits::new(1<<5);

    pub const WDCE: RegisterBits<Self> = RegisterBits::new(0x10);
    pub const WDCE0: RegisterBits<Self> = RegisterBits::new(1<<4);

    pub const WDE: RegisterBits<Self> = RegisterBits::new(0x8);
    pub const WDE0: RegisterBits<Self> = RegisterBits::new(1<<3);

}

impl Register for WDTCSR {
    type T = u8;
    const ADDRESS: *mut u8 = 0x60 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct EIND;

impl EIND {
}

impl Register for EIND {
    type T = u8;
    const ADDRESS: *mut u8 = 0x5c as *mut u8;
}
pub mod port {
    #![allow(unused_imports)]

    use super::*;
    use crate::Pin;

    pub struct B0;

    impl Pin for B0 {
        /// Port B Data Register.
        type PORT = PORTB;
        /// Port B Data Direction Register.
        type DDR = DDRB;
        /// Port B Input Pins.
        type PIN = PINB;
        /// PB0
        const MASK: u8 = 1<<0;
    }

    pub struct B1;

    impl Pin for B1 {
        /// Port B Data Register.
        type PORT = PORTB;
        /// Port B Data Direction Register.
        type DDR = DDRB;
        /// Port B Input Pins.
        type PIN = PINB;
        /// PB1
        const MASK: u8 = 1<<1;
    }

    pub struct B2;

    impl Pin for B2 {
        /// Port B Data Register.
        type PORT = PORTB;
        /// Port B Data Direction Register.
        type DDR = DDRB;
        /// Port B Input Pins.
        type PIN = PINB;
        /// PB2
        const MASK: u8 = 1<<2;
    }

    pub struct B3;

    impl Pin for B3 {
        /// Port B Data Register.
        type PORT = PORTB;
        /// Port B Data Direction Register.
        type DDR = DDRB;
        /// Port B Input Pins.
        type PIN = PINB;
        /// PB3
        const MASK: u8 = 1<<3;
    }

    pub struct B4;

    impl Pin for B4 {
        /// Port B Data Register.
        type PORT = PORTB;
        /// Port B Data Direction Register.
        type DDR = DDRB;
        /// Port B Input Pins.
        type PIN = PINB;
        /// PB4
        const MASK: u8 = 1<<4;
    }

    pub struct B5;

    impl Pin for B5 {
        /// Port B Data Register.
        type PORT = PORTB;
        /// Port B Data Direction Register.
        type DDR = DDRB;
        /// Port B Input Pins.
        type PIN = PINB;
        /// PB5
        const MASK: u8 = 1<<5;
    }

    pub struct B6;

    impl Pin for B6 {
        /// Port B Data Register.
        type PORT = PORTB;
        /// Port B Data Direction Register.
        type DDR = DDRB;
        /// Port B Input Pins.
        type PIN = PINB;
        /// PB6
        const MASK: u8 = 1<<6;
    }

    pub struct B7;

    impl Pin for B7 {
        /// Port B Data Register.
        type PORT = PORTB;
        /// Port B Data Direction Register.
        type DDR = DDRB;
        /// Port B Input Pins.
        type PIN = PINB;
        /// PB7
        const MASK: u8 = 1<<7;
    }

    pub struct C6;

    impl Pin for C6 {
        /// Port C Data Register.
        type PORT = PORTC;
        /// Port C Data Direction Register.
        type DDR = DDRC;
        /// Port C Input Pins.
        type PIN = PINC;
        /// PC6
        const MASK: u8 = 1<<6;
    }

    pub struct C7;

    impl Pin for C7 {
        /// Port C Data Register.
        type PORT = PORTC;
        /// Port C Data Direction Register.
        type DDR = DDRC;
        /// Port C Input Pins.
        type PIN = PINC;
        /// PC7
        const MASK: u8 = 1<<7;
    }

    pub struct D0;

    impl Pin for D0 {
        /// Port D Data Register.
        type PORT = PORTD;
        /// Port D Data Direction Register.
        type DDR = DDRD;
        /// Port D Input Pins.
        type PIN = PIND;
        /// PD0
        const MASK: u8 = 1<<0;
    }

    pub struct D1;

    impl Pin for D1 {
        /// Port D Data Register.
        type PORT = PORTD;
        /// Port D Data Direction Register.
        type DDR = DDRD;
        /// Port D Input Pins.
        type PIN = PIND;
        /// PD1
        const MASK: u8 = 1<<1;
    }

    pub struct D2;

    impl Pin for D2 {
        /// Port D Data Register.
        type PORT = PORTD;
        /// Port D Data Direction Register.
        type DDR = DDRD;
        /// Port D Input Pins.
        type PIN = PIND;
        /// PD2
        const MASK: u8 = 1<<2;
    }

    pub struct D3;

    impl Pin for D3 {
        /// Port D Data Register.
        type PORT = PORTD;
        /// Port D Data Direction Register.
        type DDR = DDRD;
        /// Port D Input Pins.
        type PIN = PIND;
        /// PD3
        const MASK: u8 = 1<<3;
    }

    pub struct D4;

    impl Pin for D4 {
        /// Port D Data Register.
        type PORT = PORTD;
        /// Port D Data Direction Register.
        type DDR = DDRD;
        /// Port D Input Pins.
        type PIN = PIND;
        /// PD4
        const MASK: u8 = 1<<4;
    }

    pub struct D5;

    impl Pin for D5 {
        /// Port D Data Register.
        type PORT = PORTD;
        /// Port D Data Direction Register.
        type DDR = DDRD;
        /// Port D Input Pins.
        type PIN = PIND;
        /// PD5
        const MASK: u8 = 1<<5;
    }

    pub struct D6;

    impl Pin for D6 {
        /// Port D Data Register.
        type PORT = PORTD;
        /// Port D Data Direction Register.
        type DDR = DDRD;
        /// Port D Input Pins.
        type PIN = PIND;
        /// PD6
        const MASK: u8 = 1<<6;
    }

    pub struct D7;

    impl Pin for D7 {
        /// Port D Data Register.
        type PORT = PORTD;
        /// Port D Data Direction Register.
        type DDR = DDRD;
        /// Port D Input Pins.
        type PIN = PIND;
        /// PD7
        const MASK: u8 = 1<<7;
    }

    pub struct E2;

    impl Pin for E2 {
        /// Data Register, Port E.
        type PORT = PORTE;
        /// Data Direction Register, Port E.
        type DDR = DDRE;
        /// Input Pins, Port E.
        type PIN = PINE;
        /// PE2
        const MASK: u8 = 1<<2;
    }

    pub struct E6;

    impl Pin for E6 {
        /// Data Register, Port E.
        type PORT = PORTE;
        /// Data Direction Register, Port E.
        type DDR = DDRE;
        /// Input Pins, Port E.
        type PIN = PINE;
        /// PE6
        const MASK: u8 = 1<<6;
    }

    pub struct F0;

    impl Pin for F0 {
        /// Data Register, Port F.
        type PORT = PORTF;
        /// Data Direction Register, Port F.
        type DDR = DDRF;
        /// Input Pins, Port F.
        type PIN = PINF;
        /// PF0
        const MASK: u8 = 1<<0;
    }

    pub struct F1;

    impl Pin for F1 {
        /// Data Register, Port F.
        type PORT = PORTF;
        /// Data Direction Register, Port F.
        type DDR = DDRF;
        /// Input Pins, Port F.
        type PIN = PINF;
        /// PF1
        const MASK: u8 = 1<<1;
    }

    pub struct F4;

    impl Pin for F4 {
        /// Data Register, Port F.
        type PORT = PORTF;
        /// Data Direction Register, Port F.
        type DDR = DDRF;
        /// Input Pins, Port F.
        type PIN = PINF;
        /// PF4
        const MASK: u8 = 1<<4;
    }

    pub struct F5;

    impl Pin for F5 {
        /// Data Register, Port F.
        type PORT = PORTF;
        /// Data Direction Register, Port F.
        type DDR = DDRF;
        /// Input Pins, Port F.
        type PIN = PINF;
        /// PF5
        const MASK: u8 = 1<<5;
    }

    pub struct F6;

    impl Pin for F6 {
        /// Data Register, Port F.
        type PORT = PORTF;
        /// Data Direction Register, Port F.
        type DDR = DDRF;
        /// Input Pins, Port F.
        type PIN = PINF;
        /// PF6
        const MASK: u8 = 1<<6;
    }

}

pub struct Spi;

impl modules::HardwareSpi for Spi {
    type SlaveSelect = port::B0;
    type Clock = port::B1;
    type MasterOutSlaveIn = port::B2;
    type MasterInSlaveOut = port::B3;
    type ControlRegister = SPCR;
    type StatusRegister = SPSR;
    type DataRegister = SPDR;
}

/// The USART1 module.
pub struct USART1;

impl modules::HardwareUsart for USART1 {
    type DataRegister = UDR1;
    type ControlRegisterA = UCSR1A;
    type ControlRegisterB = UCSR1B;
    type ControlRegisterC = UCSR1C;
    type BaudRateRegister = UBRR1;
}

/// 8-bit timer.
pub struct Timer8;

impl modules::Timer8 for Timer8 {
    type CompareA = OCR0A;
    type CompareB = OCR0B;
    type Counter = TCNT0;
    type ControlA = TCCR0A;
    type ControlB = TCCR0B;
    type InterruptMask = TIMSK0;
    type InterruptFlag = TIFR0;
    const CS0: RegisterBits<Self::ControlB> = Self::ControlB::CS00;
    const CS1: RegisterBits<Self::ControlB> = Self::ControlB::CS01;
    const CS2: RegisterBits<Self::ControlB> = Self::ControlB::CS02;
    const WGM0: RegisterBits<Self::ControlA> = Self::ControlA::WGM00;
    const WGM1: RegisterBits<Self::ControlA> = Self::ControlA::WGM01;
    const WGM2: RegisterBits<Self::ControlB> = Self::ControlB::WGM020;
    const OCIEA: RegisterBits<Self::InterruptMask> = Self::InterruptMask::OCIE0A;
}
/// 16-bit timer.
pub struct Timer16;

impl modules::Timer16 for Timer16 {
    type CompareA = OCR3A;
    type CompareB = OCR3B;
    type Counter = TCNT3;
    type ControlA = TCCR3A;
    type ControlB = TCCR3B;
    type ControlC = TCCR3C;
    type InterruptMask = TIMSK3;
    type InterruptFlag = TIFR3;
    const CS0: RegisterBits<Self::ControlB> = Self::ControlB::CS30;
    const CS1: RegisterBits<Self::ControlB> = Self::ControlB::CS31;
    const CS2: RegisterBits<Self::ControlB> = Self::ControlB::CS32;
    const WGM0: RegisterBits<Self::ControlA> = Self::ControlA::WGM30;
    const WGM1: RegisterBits<Self::ControlA> = Self::ControlA::WGM31;
    const WGM2: RegisterBits<Self::ControlB> = Self::ControlB::WGM30;
    const WGM3: RegisterBits<Self::ControlB> = Self::ControlB::WGM31;
    const OCIEA: RegisterBits<Self::InterruptMask> = Self::InterruptMask::OCIE3A;
}

