//! Routines for managing interrupts.

use core::arch::asm;
use core::prelude::v1::*;
use core::marker::PhantomData;

/// Helper struct that automatically restores interrupts
/// on drop.
struct DisableInterrupts(PhantomData<()>);

/// Executes a closure, disabling interrupts until its completion.
///
/// Restores interrupts after the closure has completed
/// execution.
#[inline(always)]
pub fn without_interrupts<F, T>(f: F) -> T
    where F: FnOnce() -> T
{
    let _disabled = DisableInterrupts::new();
    f()
}

impl DisableInterrupts {
    #[inline(always)]
    pub fn new() -> DisableInterrupts {
        unsafe { asm!("CLI") }
        DisableInterrupts(PhantomData)
    }
}

impl Drop for DisableInterrupts {
    #[inline(always)]
    fn drop(&mut self) {
        unsafe { asm!("SEI") }
    }
}

