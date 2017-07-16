use core::prelude::v1::*;
use core::marker::PhantomData;

pub use io::{PORT_B, PORT_C, PORT_D};

pub struct DisableInterrupts(PhantomData<()>);

impl DisableInterrupts {
    #[inline]
    pub fn new() -> DisableInterrupts {
        unsafe { asm!("CLI") }
        DisableInterrupts(PhantomData)
    }
}

impl Drop for DisableInterrupts {
    #[inline]
    fn drop(&mut self) {
        unsafe { asm!("SEI") }
    }
}

pub fn without_interrupts<F, T>(f: F) -> T
    where F: FnOnce() -> T
{
    let _disabled = DisableInterrupts::new();
    f()
}
