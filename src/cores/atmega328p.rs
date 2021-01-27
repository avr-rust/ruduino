//! Core for ATmega328P.

use crate::{modules, RegisterBits, Register};

#[allow(non_camel_case_types)]
pub struct EXTENDED;

impl EXTENDED {
    pub const BODLEVEL: RegisterBits<Self> = RegisterBits::new(0x7);
    pub const BODLEVEL0: RegisterBits<Self> = RegisterBits::new(1<<0);
    pub const BODLEVEL1: RegisterBits<Self> = RegisterBits::new(1<<1);
    pub const BODLEVEL2: RegisterBits<Self> = RegisterBits::new(1<<2);

}

impl Register for EXTENDED {
    type T = u8;
    const ADDRESS: *mut u8 = 0x2 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct HIGH;

impl HIGH {
    pub const RSTDISBL: RegisterBits<Self> = RegisterBits::new(0x80);
    pub const RSTDISBL0: RegisterBits<Self> = RegisterBits::new(1<<7);

    pub const DWEN: RegisterBits<Self> = RegisterBits::new(0x40);
    pub const DWEN0: RegisterBits<Self> = RegisterBits::new(1<<6);

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
pub struct LOW;

impl LOW {
    pub const CKDIV8: RegisterBits<Self> = RegisterBits::new(0x80);
    pub const CKDIV80: RegisterBits<Self> = RegisterBits::new(1<<7);

    pub const CKOUT: RegisterBits<Self> = RegisterBits::new(0x40);
    pub const CKOUT0: RegisterBits<Self> = RegisterBits::new(1<<6);

    pub const SUT_CKSEL: RegisterBits<Self> = RegisterBits::new(0x3f);
    pub const SUT_CKSEL0: RegisterBits<Self> = RegisterBits::new(1<<0);
    pub const SUT_CKSEL1: RegisterBits<Self> = RegisterBits::new(1<<1);
    pub const SUT_CKSEL2: RegisterBits<Self> = RegisterBits::new(1<<2);
    pub const SUT_CKSEL3: RegisterBits<Self> = RegisterBits::new(1<<3);
    pub const SUT_CKSEL4: RegisterBits<Self> = RegisterBits::new(1<<4);
    pub const SUT_CKSEL5: RegisterBits<Self> = RegisterBits::new(1<<5);

}

impl Register for LOW {
    type T = u8;
    const ADDRESS: *mut u8 = 0x0 as *mut u8;
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
pub struct UDR0;

impl UDR0 {
}

impl Register for UDR0 {
    type T = u8;
    const ADDRESS: *mut u8 = 0xc6 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct UCSR0A;

impl UCSR0A {
    pub const RXC0: RegisterBits<Self> = RegisterBits::new(0x80);
    pub const RXC00: RegisterBits<Self> = RegisterBits::new(1<<7);

    pub const TXC0: RegisterBits<Self> = RegisterBits::new(0x40);
    pub const TXC00: RegisterBits<Self> = RegisterBits::new(1<<6);

    pub const UDRE0: RegisterBits<Self> = RegisterBits::new(0x20);
    pub const UDRE00: RegisterBits<Self> = RegisterBits::new(1<<5);

    pub const FE0: RegisterBits<Self> = RegisterBits::new(0x10);
    pub const FE00: RegisterBits<Self> = RegisterBits::new(1<<4);

    pub const DOR0: RegisterBits<Self> = RegisterBits::new(0x8);
    pub const DOR00: RegisterBits<Self> = RegisterBits::new(1<<3);

    pub const UPE0: RegisterBits<Self> = RegisterBits::new(0x4);
    pub const UPE00: RegisterBits<Self> = RegisterBits::new(1<<2);

    pub const U2X0: RegisterBits<Self> = RegisterBits::new(0x2);
    pub const U2X00: RegisterBits<Self> = RegisterBits::new(1<<1);

    pub const MPCM0: RegisterBits<Self> = RegisterBits::new(0x1);
    pub const MPCM00: RegisterBits<Self> = RegisterBits::new(1<<0);

}

impl Register for UCSR0A {
    type T = u8;
    const ADDRESS: *mut u8 = 0xc0 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct UCSR0B;

impl UCSR0B {
    pub const RXCIE0: RegisterBits<Self> = RegisterBits::new(0x80);
    pub const RXCIE00: RegisterBits<Self> = RegisterBits::new(1<<7);

    pub const TXCIE0: RegisterBits<Self> = RegisterBits::new(0x40);
    pub const TXCIE00: RegisterBits<Self> = RegisterBits::new(1<<6);

    pub const UDRIE0: RegisterBits<Self> = RegisterBits::new(0x20);
    pub const UDRIE00: RegisterBits<Self> = RegisterBits::new(1<<5);

    pub const RXEN0: RegisterBits<Self> = RegisterBits::new(0x10);
    pub const RXEN00: RegisterBits<Self> = RegisterBits::new(1<<4);

    pub const TXEN0: RegisterBits<Self> = RegisterBits::new(0x8);
    pub const TXEN00: RegisterBits<Self> = RegisterBits::new(1<<3);

    pub const UCSZ02: RegisterBits<Self> = RegisterBits::new(0x4);
    pub const UCSZ020: RegisterBits<Self> = RegisterBits::new(1<<2);

    pub const RXB80: RegisterBits<Self> = RegisterBits::new(0x2);
    pub const RXB800: RegisterBits<Self> = RegisterBits::new(1<<1);

    pub const TXB80: RegisterBits<Self> = RegisterBits::new(0x1);
    pub const TXB800: RegisterBits<Self> = RegisterBits::new(1<<0);

}

impl Register for UCSR0B {
    type T = u8;
    const ADDRESS: *mut u8 = 0xc1 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct UCSR0C;

impl UCSR0C {
    pub const UMSEL0: RegisterBits<Self> = RegisterBits::new(0xc0);
    pub const UMSEL00: RegisterBits<Self> = RegisterBits::new(1<<6);
    pub const UMSEL01: RegisterBits<Self> = RegisterBits::new(1<<7);

    pub const UPM0: RegisterBits<Self> = RegisterBits::new(0x30);
    pub const UPM00: RegisterBits<Self> = RegisterBits::new(1<<4);
    pub const UPM01: RegisterBits<Self> = RegisterBits::new(1<<5);

    pub const USBS0: RegisterBits<Self> = RegisterBits::new(0x8);
    pub const USBS00: RegisterBits<Self> = RegisterBits::new(1<<3);

    pub const UCSZ0: RegisterBits<Self> = RegisterBits::new(0x6);
    pub const UCSZ00: RegisterBits<Self> = RegisterBits::new(1<<1);
    pub const UCSZ01: RegisterBits<Self> = RegisterBits::new(1<<2);

    pub const UCPOL0: RegisterBits<Self> = RegisterBits::new(0x1);
    pub const UCPOL00: RegisterBits<Self> = RegisterBits::new(1<<0);

}

impl Register for UCSR0C {
    type T = u8;
    const ADDRESS: *mut u8 = 0xc2 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct UBRR0;

impl UBRR0 {
}

impl Register for UBRR0 {
    type T = u16;
    const ADDRESS: *mut u16 = 0xc4 as *mut u16;
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
pub struct TWBR;

impl TWBR {
}

impl Register for TWBR {
    type T = u8;
    const ADDRESS: *mut u8 = 0xb8 as *mut u8;
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
pub struct TWDR;

impl TWDR {
}

impl Register for TWDR {
    type T = u8;
    const ADDRESS: *mut u8 = 0xbb as *mut u8;
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
pub struct TIMSK1;

impl TIMSK1 {
    pub const ICIE1: RegisterBits<Self> = RegisterBits::new(0x20);
    pub const ICIE10: RegisterBits<Self> = RegisterBits::new(1<<5);

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
pub struct TIFR1;

impl TIFR1 {
    pub const ICF1: RegisterBits<Self> = RegisterBits::new(0x20);
    pub const ICF10: RegisterBits<Self> = RegisterBits::new(1<<5);

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
pub struct TCCR1A;

impl TCCR1A {
    pub const COM1A: RegisterBits<Self> = RegisterBits::new(0xc0);
    pub const COM1A0: RegisterBits<Self> = RegisterBits::new(1<<6);
    pub const COM1A1: RegisterBits<Self> = RegisterBits::new(1<<7);

    pub const COM1B: RegisterBits<Self> = RegisterBits::new(0x30);
    pub const COM1B0: RegisterBits<Self> = RegisterBits::new(1<<4);
    pub const COM1B1: RegisterBits<Self> = RegisterBits::new(1<<5);

    pub const WGM1: RegisterBits<Self> = RegisterBits::new(0x3);
    pub const WGM10: RegisterBits<Self> = RegisterBits::new(1<<0);
    pub const WGM11: RegisterBits<Self> = RegisterBits::new(1<<1);

}

impl Register for TCCR1A {
    type T = u8;
    const ADDRESS: *mut u8 = 0x80 as *mut u8;
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
pub struct TCCR1C;

impl TCCR1C {
    pub const FOC1A: RegisterBits<Self> = RegisterBits::new(0x80);
    pub const FOC1A0: RegisterBits<Self> = RegisterBits::new(1<<7);

    pub const FOC1B: RegisterBits<Self> = RegisterBits::new(0x40);
    pub const FOC1B0: RegisterBits<Self> = RegisterBits::new(1<<6);

}

impl Register for TCCR1C {
    type T = u8;
    const ADDRESS: *mut u8 = 0x82 as *mut u8;
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
pub struct OCR1A;

impl OCR1A {
}

impl Register for OCR1A {
    type T = u16;
    const ADDRESS: *mut u16 = 0x88 as *mut u16;
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
pub struct ICR1;

impl ICR1 {
}

impl Register for ICR1 {
    type T = u16;
    const ADDRESS: *mut u16 = 0x86 as *mut u16;
}
#[allow(non_camel_case_types)]
pub struct TIMSK2;

impl TIMSK2 {
    pub const OCIE2B: RegisterBits<Self> = RegisterBits::new(0x4);
    pub const OCIE2B0: RegisterBits<Self> = RegisterBits::new(1<<2);

    pub const OCIE2A: RegisterBits<Self> = RegisterBits::new(0x2);
    pub const OCIE2A0: RegisterBits<Self> = RegisterBits::new(1<<1);

    pub const TOIE2: RegisterBits<Self> = RegisterBits::new(0x1);
    pub const TOIE20: RegisterBits<Self> = RegisterBits::new(1<<0);

}

impl Register for TIMSK2 {
    type T = u8;
    const ADDRESS: *mut u8 = 0x70 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct TIFR2;

impl TIFR2 {
    pub const OCF2B: RegisterBits<Self> = RegisterBits::new(0x4);
    pub const OCF2B0: RegisterBits<Self> = RegisterBits::new(1<<2);

    pub const OCF2A: RegisterBits<Self> = RegisterBits::new(0x2);
    pub const OCF2A0: RegisterBits<Self> = RegisterBits::new(1<<1);

    pub const TOV2: RegisterBits<Self> = RegisterBits::new(0x1);
    pub const TOV20: RegisterBits<Self> = RegisterBits::new(1<<0);

}

impl Register for TIFR2 {
    type T = u8;
    const ADDRESS: *mut u8 = 0x37 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct TCCR2A;

impl TCCR2A {
    pub const COM2A: RegisterBits<Self> = RegisterBits::new(0xc0);
    pub const COM2A0: RegisterBits<Self> = RegisterBits::new(1<<6);
    pub const COM2A1: RegisterBits<Self> = RegisterBits::new(1<<7);

    pub const COM2B: RegisterBits<Self> = RegisterBits::new(0x30);
    pub const COM2B0: RegisterBits<Self> = RegisterBits::new(1<<4);
    pub const COM2B1: RegisterBits<Self> = RegisterBits::new(1<<5);

    pub const WGM2: RegisterBits<Self> = RegisterBits::new(0x3);
    pub const WGM20: RegisterBits<Self> = RegisterBits::new(1<<0);
    pub const WGM21: RegisterBits<Self> = RegisterBits::new(1<<1);

}

impl Register for TCCR2A {
    type T = u8;
    const ADDRESS: *mut u8 = 0xb0 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct TCCR2B;

impl TCCR2B {
    pub const FOC2A: RegisterBits<Self> = RegisterBits::new(0x80);
    pub const FOC2A0: RegisterBits<Self> = RegisterBits::new(1<<7);

    pub const FOC2B: RegisterBits<Self> = RegisterBits::new(0x40);
    pub const FOC2B0: RegisterBits<Self> = RegisterBits::new(1<<6);

    pub const WGM22: RegisterBits<Self> = RegisterBits::new(0x8);
    pub const WGM220: RegisterBits<Self> = RegisterBits::new(1<<3);

    pub const CS2: RegisterBits<Self> = RegisterBits::new(0x7);
    pub const CS20: RegisterBits<Self> = RegisterBits::new(1<<0);
    pub const CS21: RegisterBits<Self> = RegisterBits::new(1<<1);
    pub const CS22: RegisterBits<Self> = RegisterBits::new(1<<2);

}

impl Register for TCCR2B {
    type T = u8;
    const ADDRESS: *mut u8 = 0xb1 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct TCNT2;

impl TCNT2 {
}

impl Register for TCNT2 {
    type T = u8;
    const ADDRESS: *mut u8 = 0xb2 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct OCR2B;

impl OCR2B {
}

impl Register for OCR2B {
    type T = u8;
    const ADDRESS: *mut u8 = 0xb4 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct OCR2A;

impl OCR2A {
}

impl Register for OCR2A {
    type T = u8;
    const ADDRESS: *mut u8 = 0xb3 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct ASSR;

impl ASSR {
    pub const EXCLK: RegisterBits<Self> = RegisterBits::new(0x40);
    pub const EXCLK0: RegisterBits<Self> = RegisterBits::new(1<<6);

    pub const AS2: RegisterBits<Self> = RegisterBits::new(0x20);
    pub const AS20: RegisterBits<Self> = RegisterBits::new(1<<5);

    pub const TCN2UB: RegisterBits<Self> = RegisterBits::new(0x10);
    pub const TCN2UB0: RegisterBits<Self> = RegisterBits::new(1<<4);

    pub const OCR2AUB: RegisterBits<Self> = RegisterBits::new(0x8);
    pub const OCR2AUB0: RegisterBits<Self> = RegisterBits::new(1<<3);

    pub const OCR2BUB: RegisterBits<Self> = RegisterBits::new(0x4);
    pub const OCR2BUB0: RegisterBits<Self> = RegisterBits::new(1<<2);

    pub const TCR2AUB: RegisterBits<Self> = RegisterBits::new(0x2);
    pub const TCR2AUB0: RegisterBits<Self> = RegisterBits::new(1<<1);

    pub const TCR2BUB: RegisterBits<Self> = RegisterBits::new(0x1);
    pub const TCR2BUB0: RegisterBits<Self> = RegisterBits::new(1<<0);

}

impl Register for ASSR {
    type T = u8;
    const ADDRESS: *mut u8 = 0xb6 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct ADMUX;

impl ADMUX {
    pub const REFS: RegisterBits<Self> = RegisterBits::new(0xc0);
    pub const REFS0: RegisterBits<Self> = RegisterBits::new(1<<6);
    pub const REFS1: RegisterBits<Self> = RegisterBits::new(1<<7);

    pub const ADLAR: RegisterBits<Self> = RegisterBits::new(0x20);
    pub const ADLAR0: RegisterBits<Self> = RegisterBits::new(1<<5);

    pub const MUX: RegisterBits<Self> = RegisterBits::new(0xf);
    pub const MUX0: RegisterBits<Self> = RegisterBits::new(1<<0);
    pub const MUX1: RegisterBits<Self> = RegisterBits::new(1<<1);
    pub const MUX2: RegisterBits<Self> = RegisterBits::new(1<<2);
    pub const MUX3: RegisterBits<Self> = RegisterBits::new(1<<3);

}

impl Register for ADMUX {
    type T = u8;
    const ADDRESS: *mut u8 = 0x7c as *mut u8;
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
pub struct ADCSRB;

impl ADCSRB {
    pub const ACME: RegisterBits<Self> = RegisterBits::new(0x40);
    pub const ACME0: RegisterBits<Self> = RegisterBits::new(1<<6);

    pub const ADTS: RegisterBits<Self> = RegisterBits::new(0x7);
    pub const ADTS0: RegisterBits<Self> = RegisterBits::new(1<<0);
    pub const ADTS1: RegisterBits<Self> = RegisterBits::new(1<<1);
    pub const ADTS2: RegisterBits<Self> = RegisterBits::new(1<<2);

}

impl Register for ADCSRB {
    type T = u8;
    const ADDRESS: *mut u8 = 0x7b as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct DIDR0;

impl DIDR0 {
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
pub struct PORTB;

impl PORTB {
}

impl Register for PORTB {
    type T = u8;
    const ADDRESS: *mut u8 = 0x25 as *mut u8;
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
pub struct PINB;

impl PINB {
}

impl Register for PINB {
    type T = u8;
    const ADDRESS: *mut u8 = 0x23 as *mut u8;
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
pub struct DDRC;

impl DDRC {
}

impl Register for DDRC {
    type T = u8;
    const ADDRESS: *mut u8 = 0x27 as *mut u8;
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
pub struct PORTD;

impl PORTD {
}

impl Register for PORTD {
    type T = u8;
    const ADDRESS: *mut u8 = 0x2b as *mut u8;
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
pub struct PIND;

impl PIND {
}

impl Register for PIND {
    type T = u8;
    const ADDRESS: *mut u8 = 0x29 as *mut u8;
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
pub struct OCR0A;

impl OCR0A {
}

impl Register for OCR0A {
    type T = u8;
    const ADDRESS: *mut u8 = 0x47 as *mut u8;
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
pub struct EICRA;

impl EICRA {
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
pub struct EIMSK;

impl EIMSK {
    pub const INT: RegisterBits<Self> = RegisterBits::new(0x3);
    pub const INT0: RegisterBits<Self> = RegisterBits::new(1<<0);
    pub const INT1: RegisterBits<Self> = RegisterBits::new(1<<1);

}

impl Register for EIMSK {
    type T = u8;
    const ADDRESS: *mut u8 = 0x3d as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct EIFR;

impl EIFR {
    pub const INTF: RegisterBits<Self> = RegisterBits::new(0x3);
    pub const INTF0: RegisterBits<Self> = RegisterBits::new(1<<0);
    pub const INTF1: RegisterBits<Self> = RegisterBits::new(1<<1);

}

impl Register for EIFR {
    type T = u8;
    const ADDRESS: *mut u8 = 0x3c as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct PCICR;

impl PCICR {
    pub const PCIE: RegisterBits<Self> = RegisterBits::new(0x7);
    pub const PCIE0: RegisterBits<Self> = RegisterBits::new(1<<0);
    pub const PCIE1: RegisterBits<Self> = RegisterBits::new(1<<1);
    pub const PCIE2: RegisterBits<Self> = RegisterBits::new(1<<2);

}

impl Register for PCICR {
    type T = u8;
    const ADDRESS: *mut u8 = 0x68 as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct PCMSK2;

impl PCMSK2 {
    pub const PCINT: RegisterBits<Self> = RegisterBits::new(0xff);
    pub const PCINT0: RegisterBits<Self> = RegisterBits::new(1<<0);
    pub const PCINT1: RegisterBits<Self> = RegisterBits::new(1<<1);
    pub const PCINT2: RegisterBits<Self> = RegisterBits::new(1<<2);
    pub const PCINT3: RegisterBits<Self> = RegisterBits::new(1<<3);
    pub const PCINT4: RegisterBits<Self> = RegisterBits::new(1<<4);
    pub const PCINT5: RegisterBits<Self> = RegisterBits::new(1<<5);
    pub const PCINT6: RegisterBits<Self> = RegisterBits::new(1<<6);
    pub const PCINT7: RegisterBits<Self> = RegisterBits::new(1<<7);

}

impl Register for PCMSK2 {
    type T = u8;
    const ADDRESS: *mut u8 = 0x6d as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct PCMSK1;

impl PCMSK1 {
    pub const PCINT: RegisterBits<Self> = RegisterBits::new(0x7f);
    pub const PCINT0: RegisterBits<Self> = RegisterBits::new(1<<0);
    pub const PCINT1: RegisterBits<Self> = RegisterBits::new(1<<1);
    pub const PCINT2: RegisterBits<Self> = RegisterBits::new(1<<2);
    pub const PCINT3: RegisterBits<Self> = RegisterBits::new(1<<3);
    pub const PCINT4: RegisterBits<Self> = RegisterBits::new(1<<4);
    pub const PCINT5: RegisterBits<Self> = RegisterBits::new(1<<5);
    pub const PCINT6: RegisterBits<Self> = RegisterBits::new(1<<6);

}

impl Register for PCMSK1 {
    type T = u8;
    const ADDRESS: *mut u8 = 0x6c as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct PCMSK0;

impl PCMSK0 {
    pub const PCINT: RegisterBits<Self> = RegisterBits::new(0xff);
    pub const PCINT0: RegisterBits<Self> = RegisterBits::new(1<<0);
    pub const PCINT1: RegisterBits<Self> = RegisterBits::new(1<<1);
    pub const PCINT2: RegisterBits<Self> = RegisterBits::new(1<<2);
    pub const PCINT3: RegisterBits<Self> = RegisterBits::new(1<<3);
    pub const PCINT4: RegisterBits<Self> = RegisterBits::new(1<<4);
    pub const PCINT5: RegisterBits<Self> = RegisterBits::new(1<<5);
    pub const PCINT6: RegisterBits<Self> = RegisterBits::new(1<<6);
    pub const PCINT7: RegisterBits<Self> = RegisterBits::new(1<<7);

}

impl Register for PCMSK0 {
    type T = u8;
    const ADDRESS: *mut u8 = 0x6b as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct PCIFR;

impl PCIFR {
    pub const PCIF: RegisterBits<Self> = RegisterBits::new(0x7);
    pub const PCIF0: RegisterBits<Self> = RegisterBits::new(1<<0);
    pub const PCIF1: RegisterBits<Self> = RegisterBits::new(1<<1);
    pub const PCIF2: RegisterBits<Self> = RegisterBits::new(1<<2);

}

impl Register for PCIFR {
    type T = u8;
    const ADDRESS: *mut u8 = 0x3b as *mut u8;
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
pub struct PRR;

impl PRR {
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

impl Register for PRR {
    type T = u8;
    const ADDRESS: *mut u8 = 0x64 as *mut u8;
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
pub struct SP;

impl SP {
}

impl Register for SP {
    type T = u16;
    const ADDRESS: *mut u16 = 0x5d as *mut u16;
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
pub struct MCUCR;

impl MCUCR {
    pub const BODS: RegisterBits<Self> = RegisterBits::new(0x40);
    pub const BODS0: RegisterBits<Self> = RegisterBits::new(1<<6);

    pub const BODSE: RegisterBits<Self> = RegisterBits::new(0x20);
    pub const BODSE0: RegisterBits<Self> = RegisterBits::new(1<<5);

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
pub struct MCUSR;

impl MCUSR {
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
pub struct GPIOR2;

impl GPIOR2 {
}

impl Register for GPIOR2 {
    type T = u8;
    const ADDRESS: *mut u8 = 0x4b as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct GPIOR1;

impl GPIOR1 {
}

impl Register for GPIOR1 {
    type T = u8;
    const ADDRESS: *mut u8 = 0x4a as *mut u8;
}
#[allow(non_camel_case_types)]
pub struct GPIOR0;

impl GPIOR0 {
}

impl Register for GPIOR0 {
    type T = u8;
    const ADDRESS: *mut u8 = 0x3e as *mut u8;
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

    pub struct C0;

    impl Pin for C0 {
        /// Port C Data Register.
        type PORT = PORTC;
        /// Port C Data Direction Register.
        type DDR = DDRC;
        /// Port C Input Pins.
        type PIN = PINC;
        /// PC0
        const MASK: u8 = 1<<0;
    }

    pub struct C1;

    impl Pin for C1 {
        /// Port C Data Register.
        type PORT = PORTC;
        /// Port C Data Direction Register.
        type DDR = DDRC;
        /// Port C Input Pins.
        type PIN = PINC;
        /// PC1
        const MASK: u8 = 1<<1;
    }

    pub struct C2;

    impl Pin for C2 {
        /// Port C Data Register.
        type PORT = PORTC;
        /// Port C Data Direction Register.
        type DDR = DDRC;
        /// Port C Input Pins.
        type PIN = PINC;
        /// PC2
        const MASK: u8 = 1<<2;
    }

    pub struct C3;

    impl Pin for C3 {
        /// Port C Data Register.
        type PORT = PORTC;
        /// Port C Data Direction Register.
        type DDR = DDRC;
        /// Port C Input Pins.
        type PIN = PINC;
        /// PC3
        const MASK: u8 = 1<<3;
    }

    pub struct C4;

    impl Pin for C4 {
        /// Port C Data Register.
        type PORT = PORTC;
        /// Port C Data Direction Register.
        type DDR = DDRC;
        /// Port C Input Pins.
        type PIN = PINC;
        /// PC4
        const MASK: u8 = 1<<4;
    }

    pub struct C5;

    impl Pin for C5 {
        /// Port C Data Register.
        type PORT = PORTC;
        /// Port C Data Direction Register.
        type DDR = DDRC;
        /// Port C Input Pins.
        type PIN = PINC;
        /// PC5
        const MASK: u8 = 1<<5;
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

}

pub struct Spi;

impl modules::HardwareSpi for Spi {
    type SlaveSelect = port::B2;
    type MasterOutSlaveIn = port::B3;
    type MasterInSlaveOut = port::B4;
    type Clock = port::B5;
    type DataRegister = SPDR;
    type StatusRegister = SPSR;
    type ControlRegister = SPCR;
}

/// The USART0 module.
pub struct USART0;

impl modules::HardwareUsart for USART0 {
    type DataRegister = UDR0;
    type ControlRegisterA = UCSR0A;
    type ControlRegisterB = UCSR0B;
    type ControlRegisterC = UCSR0C;
    type BaudRateRegister = UBRR0;
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
    type CompareA = OCR1A;
    type CompareB = OCR1B;
    type Counter = TCNT1;
    type ControlA = TCCR1A;
    type ControlB = TCCR1B;
    type ControlC = TCCR1C;
    type InterruptMask = TIMSK1;
    type InterruptFlag = TIFR1;
    const CS0: RegisterBits<Self::ControlB> = Self::ControlB::CS10;
    const CS1: RegisterBits<Self::ControlB> = Self::ControlB::CS11;
    const CS2: RegisterBits<Self::ControlB> = Self::ControlB::CS12;
    const WGM0: RegisterBits<Self::ControlA> = Self::ControlA::WGM10;
    const WGM1: RegisterBits<Self::ControlA> = Self::ControlA::WGM11;
    const WGM2: RegisterBits<Self::ControlB> = Self::ControlB::WGM10;
    const WGM3: RegisterBits<Self::ControlB> = Self::ControlB::WGM11;
    const OCIEA: RegisterBits<Self::InterruptMask> = Self::InterruptMask::OCIE1A;
}

