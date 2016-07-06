//! Definitions of register addresses and bits within those registers

#![feature(asm)]
#![feature(no_core)]
#![no_core]

extern crate core;

pub mod prelude;
pub mod timer0;
pub mod timer1;
pub mod serial;

macro_rules! bit {
    (-, $pos:expr) => {};
    ($name:ident, $pos:expr) => {
        pub const $name: u8 = 1 << $pos;
    };
}

macro_rules! register {
    ($address:expr, $name:ident, [$b7:tt, $b6:tt, $b5:tt, $b4:tt, $b3:tt, $b2:tt, $b1:tt, $b0:tt]) => {
        register!($address, $name);
        bit!($b7, 7);
        bit!($b6, 6);
        bit!($b5, 5);
        bit!($b4, 4);
        bit!($b3, 3);
        bit!($b2, 2);
        bit!($b1, 1);
        bit!($b0, 0);
    };
    ($address:expr, $name:ident) => {
        pub const $name: *mut u8 = $address as *mut u8;
    };
}

register!(0xC6, UDR0                                                                            );
register!(0xC5, UBRR0H                                                                          );
register!(0xC4, UBRR0L                                                                          );
register!(0xC2, UCSR0C, [UMSEL01, UMSEL00, UPM01,   UPM00,   USBS0,   UCSZ01,  UCSZ00,  UCPOL0 ]);
register!(0xC1, UCSR0B, [RXCIE0,  TXCIE0,  UDRIE0,  RXEN0,   TXEN0,   UCSZ02,  RXB80,   TXB80  ]);
register!(0xC0, UCSR0A, [RXC0,    TXC0,    UDRE0,   FE0,     DOR0,    UPE0,    U2X0,    MPCM0  ]);
register!(0xBD, TWAMR,  [TWAM6,   TWAM5,   TWAM4,   TWAM3,   TWAM2,   TWAM1,   TWAM0,   -      ]);
register!(0xBC, TWCR,   [TWINT,   TWEA,    TWSTA,   TWSTO,   TWWC,    TWEN,    -,       TWIE   ]);
register!(0xBB, TWDR                                                                            );
register!(0xBA, TWAR,   [TWA6,    TWA5,    TWA4,    TWA3,    TWA2,    TWA1,    TWA0,    TWGCE  ]);
register!(0xB9, TWSR,   [TWS7,    TWS6,    TWS5,    TWS4,    TWS3,    -,       TWPS1,   TWPS0  ]);
register!(0xB8, TWBR                                                                            );
register!(0xB6, ASSR,   [-,       EXCLK,   AS2,     TCN2UB,  OCR2AUB, OCR2BUB, TCR2AUB, TCR2BUB]);
register!(0xB4, OCR2B                                                                           );
register!(0xB3, OCR2A                                                                           );
register!(0xB2, TCNT2                                                                           );
register!(0xB1, TCCR2B, [FOC2A,   FOC2B,   -,       -,       WGM22,   CS22,    CS21,    CS20   ]);
register!(0xB0, TCCR2A, [COM2A1,  COM2A0,  COM2B1,  COM2B0,  -,       -,       WGM21,   WGM20  ]);
register!(0x8B, OCR1BH                                                                          );
register!(0x8A, OCR1BL                                                                          );
register!(0x89, OCR1AH                                                                          );
register!(0x88, OCR1AL                                                                          );
register!(0x87, ICR1H                                                                           );
register!(0x86, ICR1L                                                                           );
register!(0x85, TCNT1H                                                                          );
register!(0x84, TCNT1L                                                                          );
register!(0x82, TCCR1C, [FOC1A,   FOC1B,   -,       -,       -,       -,       -,       -      ]);
register!(0x81, TCCR1B, [ICNC1,   ICES1,   -,       WGM13,   WGM12,   CS12,    CS11,    CS10   ]);
register!(0x80, TCCR1A, [COM1A1,  COM1A0,  COM1B1,  COM1B0,  -,       -,       WGM11,   WGM10  ]);
register!(0x7F, DIDR1,  [-,       -,       -,       -,       -,       -,       AIN1D,   AIN0D  ]);
register!(0x7E, DIDR0,  [-,       -,       ADC5D,   ADC4D,   ADC3D,   ADC2D,   ADC1D,   ADC0D  ]);
register!(0x7C, ADMUX,  [REFS1,   REFS0,   ADLAR,   -,       MUX3,    MUX2,    MUX1,    MUX0   ]);
register!(0x7B, ADCSRB, [-,       ACME,    -,       -,       -,       ADTS2,   ADTS1,   ADTS0  ]);
register!(0x7A, ADCSRA, [ADEN,    ADSC,    ADATE,   ADIF,    ADIE,    ADPS2,   ADPS1,   ADPS0  ]);
register!(0x79, ADCH                                                                            );
register!(0x78, ADCL                                                                            );
register!(0x70, TIMSK2, [-,       -,       -,       -,       -,       OCIE2B,  OCIE2A,  TOIE2  ]);
register!(0x6F, TIMSK1, [-,       -,       ICIE1,   -,       -,       OCIE1B,  OCIE1A,  TOIE1  ]);
register!(0x6E, TIMSK0, [-,       -,       -,       -,       -,       OCIE0B,  OCIE0A,  TOIE0  ]);
register!(0x6D, PCMSK2, [PCINT23, PCINT22, PCINT21, PCINT20, PCINT19, PCINT18, PCINT17, PCINT16]);
register!(0x6C, PCMSK1, [-,       PCINT14, PCINT13, PCINT12, PCINT11, PCINT10, PCINT9,  PCINT8 ]);
register!(0x6B, PCMSK0, [PCINT7,  PCINT6,  PCINT5,  PCINT4,  PCINT3,  PCINT2,  PCINT1,  PCINT0 ]);
register!(0x69, EICRA,  [-,       -,       -,       -,       ISC11,   ISC10,   ISC01,   ISC00  ]);
register!(0x68, PCICR,  [-,       -,       -,       -,       -,       PCIE2,   PCIE1,   PCIE0  ]);
register!(0x66, OSCCAL                                                                          );
register!(0x64, PRR,    [PRTWI,   PRTIM2,  PRTIM0,  -,       PRTIM1,  PRSPI,   PRUSART0,PRADC  ]);
register!(0x61, CLKPR,  [CLKPCE,  -,       -,       -,       CLKPS3,  CLKPS2,  CLKPS1,  CLKPS0 ]);
register!(0x60, WDTCSR, [WDIF,    WDIE,    WDP3,    WDCE,    WDE,     WDP2,    WDP1,    WDP0   ]);
register!(0x5F, SREG,   [I,       T,       H,       S,       V,       N,       Z,       C      ]);
register!(0x5E, SPH,    [-,       -,       -,       -,       -,       SP10,    SP9,     SP8    ]);
register!(0x5D, SPL,    [SP7,     SP6,     SP5,     SP4,     SP3,     SP2,     SP1,     SP0    ]);
register!(0x57, SPMCSR, [SPMIE,   RWWSB,   SIGRD,   RWWSRE,  BLBSET,  PGWRT,   PGERS,   SPMEN  ]);
register!(0x55, MCUCR,  [-,       BODS,    BODSE,   PUD,     -,       -,       IVSEL,   IVCE   ]);
register!(0x54, MCUSR,  [-,       -,       -,       -,       WDRF,    BORF,    EXTRF,   PORF   ]);
register!(0x53, SMCR,   [-,       -,       -,       -,       SM2,     SM1,     SM0,     SE     ]);
register!(0x50, ACSR,   [ACD,     ACBG,    ACO,     ACI,     ACIE,    ACIC,    ACIS1,   ACIS0  ]);
register!(0x4E, SPDR                                                                            );
register!(0x4D, SPSR,   [SPIF,    WCOL,    -,       -,       -,       -,       -,       SPI2X  ]);
register!(0x4C, SPCR,   [SPIE,    SPE,     DORD,    MSTR,    CPOL,    CPHA,    SPR1,    SPR0   ]);
register!(0x4B, GPIOR2                                                                          );
register!(0x4A, GPIOR1                                                                          );
register!(0x48, OCR0B                                                                           );
register!(0x47, OCR0A                                                                           );
register!(0x46, TCNT0                                                                           );
register!(0x45, TCCR0B, [FOC0A,   FOC0B,   -,       -,       WGM02,   CS02,    CS01,    CS00   ]);
register!(0x44, TCCR0A, [COM0A1,  COM0A0,  COM0B1,  COM0B0,  -,       -,       WGM01,   WGM00  ]);
register!(0x43, GTCCR,  [TSM,     -,       -,       -,       -,       -,       PSRASY,  PSRSYNC]);
register!(0x42, EEARH                                                                           );
register!(0x41, EEARL                                                                           );
register!(0x40, EEDR                                                                            );
register!(0x3F, EECR,   [-,       -,       EEPM1,   EEPM0,   EERIE,   EEMPE,   EEPE,    EERE   ]);
register!(0x3E, GPIOR0                                                                          );
register!(0x3D, EIMSK,  [-,       -,       -,       -,       -,       -,       INT1,    INT0   ]);
register!(0x3C, EIFR,   [-,       -,       -,       -,       -,       -,       INTF1,   INTF0  ]);
register!(0x3B, PCIFR,  [-,       -,       -,       -,       -,       PCIF2,   PCIF1,   PCIF0  ]);
register!(0x37, TIFR2,  [-,       -,       -,       -,       -,       OCF2B,   OCF2A,   TOV2   ]);
register!(0x36, TIFR1,  [-,       -,       ICF1,    -,       -,       OCF1B,   OCF1A,   TOV1   ]);
register!(0x35, TIFR0,  [-,       -,       -,       -,       -,       OCF0B,   OCF0A,   TOV0   ]);
register!(0x2B, PORTD,  [PORTD7,  PORTD6,  PORTD5,  PORTD4,  PORTD3,  PORTD2,  PORTD1,  PORTD0 ]);
register!(0x2A, DDRD,   [DDD7,    DDD6,    DDD5,    DDD4,    DDD3,    DDD2,    DDD1,    DDD0   ]);
register!(0x29, PIND,   [PIND7,   PIND6,   PIND5,   PIND4,   PIND3,   PIND2,   PIND1,   PIND0  ]);
register!(0x28, PORTC,  [-,       PORTC6,  PORTC5,  PORTC4,  PORTC3,  PORTC2,  PORTC1,  PORTC0 ]);
register!(0x27, DDRC,   [-,       DDC6,    DDC5,    DDC4,    DDC3,    DDC2,    DDC1,    DDC0   ]);
register!(0x26, PINC,   [-,       PINC6,   PINC5,   PINC4,   PINC3,   PINC2,   PINC1,   PINC0  ]);
register!(0x25, PORTB,  [PORTB7,  PORTB6,  PORTB5,  PORTB4,  PORTB3,  PORTB2,  PORTB1,  PORTB0 ]);
register!(0x24, DDRB,   [DDB7,    DDB6,    DDB5,    DDB4,    DDB3,    DDB2,    DDB1,    DDB0   ]);
register!(0x23, PINB,   [PINB7,   PINB6,   PINB5,   PINB4,   PINB3,   PINB2,   PINB1,   PINB0  ]);

// 16-bit register pairs
pub const ADC:   *mut u16 = ADCL   as *mut u16;
pub const EEAR:  *mut u16 = EEARL  as *mut u16;
pub const ICR1:  *mut u16 = ICR1L  as *mut u16;
pub const OCR1A: *mut u16 = OCR1AL as *mut u16;
pub const OCR1B: *mut u16 = OCR1BL as *mut u16;
pub const OSCCA: *mut u16 = OSCCAL as *mut u16;
pub const SP:    *mut u16 = SPL    as *mut u16;
pub const TCNT1: *mut u16 = TCNT1L as *mut u16;
pub const UBRR0: *mut u16 = UBRR0L as *mut u16;

// Aliases
pub const UDORD0: u8 = UCSZ01;
pub const UCPHA0: u8 = UCSZ00;
