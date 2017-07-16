use core::prelude::v1::*;
use core::ptr::{read_volatile, write_volatile};
use core::marker::PhantomData;

use Bit;

pub trait Information {
    const DDR: *mut u8;
    const IO: *mut u8;
    const PIN: *mut u8;
}

#[derive(Copy, Clone)]
pub struct B;

impl Information for B {
    const DDR: *mut u8 = ::DDRB;
    const IO: *mut u8 = ::PORTB;
    const PIN: *mut u8 = ::PINB;
}

#[derive(Copy, Clone)]
pub struct C;

impl Information for C {
    const DDR: *mut u8 = ::DDRC;
    const IO: *mut u8 = ::PORTC;
    const PIN: *mut u8 = ::PINC;
}

#[derive(Copy, Clone)]
pub struct D;

impl Information for D {
    const DDR: *mut u8 = ::DDRD;
    const IO: *mut u8 = ::PORTD;
    const PIN: *mut u8 = ::PIND;
}

#[derive(Copy, Clone)]
enum Direction {
    Output,
    Input,
}

pub struct Configuration<P: Information> {
    _port: PhantomData<P>,
    direction: [Option<Direction>; 8],
    pullup: [Option<bool>; 8],
}

impl<P> Configuration<P>
    where
    P: Information
{
    #[inline]
    pub fn new() -> Configuration<P> {
        Configuration {
            _port: PhantomData,
            direction: Default::default(),
            pullup: Default::default(),
        }
    }

    #[inline]
    pub fn set_all_as_output(&mut self) -> &mut Self {
        self.direction = [Some(Direction::Output); 8];
        self
    }

    #[inline]
    pub fn set_all_as_input(&mut self) -> &mut Self {
        self.direction = [Some(Direction::Input); 8];
        self
    }

    #[inline]
    pub fn set_as_output(&mut self, bit: Bit) -> &mut Self {
        self.direction[bit as usize] = Some(Direction::Output);
        self
    }

    #[inline]
    pub fn set_as_input(&mut self, bit: Bit) -> &mut Self {
        self.direction[bit as usize] = Some(Direction::Input);
        self
    }

    #[inline]
    pub fn enable_pullup(&mut self, bit: Bit) -> &mut Self {
        self.pullup[bit as usize] = Some(true);
        self
    }

    #[inline]
    pub fn disable_pullup(&mut self, bit: Bit) -> &mut Self {
        self.pullup[bit as usize] = Some(false);
        self
    }

    #[inline]
    pub fn configure(&self) {
        // FIXME: Both of these loops are wasteful if we are
        // setting all 8 bits, when we could set the entire IO
        // register at once. Is there a way we can track that?

        for (i, &p) in self.direction.iter().enumerate() {
            if let Some(enabled) = p {
                if let Direction::Output = enabled {
                    unsafe { asm!("sbi $0 $1" : : "i"(P::DDR), "i"(i)) }
                } else {
                    unsafe { asm!("cbi $0 $1" : : "i"(P::DDR), "i"(i)) }
                }
            }
        }

        for (i, &p) in self.pullup.iter().enumerate() {
            if let Some(enabled) = p {
                if enabled {
                    unsafe { asm!("sbi $0 $1" : : "i"(P::IO), "i"(i)) }
                } else {
                    unsafe { asm!("cbi $0 $1" : : "i"(P::IO), "i"(i)) }
                }
            }
        }
    }
}

#[derive(Copy, Clone)]
pub struct Data<P: Information> {
    _port: PhantomData<P>,
}

impl<P> Data<P>
    where
    P: Information
{
    #[inline]
    pub fn new() -> Data<P> { Data { _port: PhantomData } }

    #[inline]
    pub fn get(&self) -> u8 {
        unsafe { read_volatile(P::PIN) }
    }

    #[inline]
    pub fn set(&self, value: u8) {
        unsafe { write_volatile(P::IO, value) };
    }

    #[inline]
    pub fn bit_is_set(&self, bit: Bit) -> bool {
        bit.is_set(self.get())
    }

    #[inline]
    pub fn set_bit(&self, bit: Bit) {
        unsafe { asm!("sbi $0 $1" : : "i"(P::IO), "i"(bit as u8)) }
    }

    #[inline]
    pub fn clear_bit(&self, bit: Bit) {
        unsafe { asm!("cbi $0 $1" : : "i"(P::IO), "i"(bit as u8)) }
    }

    #[inline]
    pub fn toggle_bit(&self, bit: Bit) {
        unsafe { asm!("sbi $0 $1" : : "i"(P::PIN), "i"(bit as u8)) }
    }
}

#[derive(Copy, Clone)]
pub struct Port<P>(PhantomData<P>);

impl<P: Information> Port<P> {
    pub(crate) const fn new() -> Port<P> { Port(PhantomData) }
    pub fn configuration(&self) -> Configuration<P> { Configuration::new() }
    pub fn data(&self) -> Data<P> { Data::new() }
}
