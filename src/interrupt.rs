//! Routines for managing interrupts.

use core::prelude::v1::*;
use core::marker::PhantomData;
use core::arch::asm;
use crate::cores::current::SREG;
use crate::register::Register;

/// Helper struct that automatically enables interrupts
/// on drop.
struct DisableInterrupts(PhantomData<()>);

/// Executes a closure, disabling interrupts until its completion.
///
/// Unconditionally enables interrupts after the closure has completed
/// execution.
#[inline(always)]
pub fn with_irqs_disabled<F, T>(f: F) -> T
    where F: FnOnce() -> T
{
    let _disabled = DisableInterrupts::new();
    f()
}

#[inline(always)]
#[deprecated(note="Please use with_irqs_disabled instead")]
pub fn without_interrupts<F, T>(f: F) -> T
    where F: FnOnce() -> T
{
    with_irqs_disabled(f)
}

impl DisableInterrupts {
    #[inline(always)]
    pub fn new() -> DisableInterrupts {
        unsafe { asm!("cli") }
        DisableInterrupts(PhantomData)
    }
}

impl Drop for DisableInterrupts {
    #[inline(always)]
    fn drop(&mut self) {
        unsafe { asm!("sei") }
    }
}

/// Helper struct that automatically restores interrupts
/// on drop.
struct SuspendInterrupts(u8);

/// Executes a closure, disabling interrupts until its completion.
///
/// Restores interrupts to the previous state after the closure has completed
/// execution.
#[inline(always)]
pub fn with_irqs_suspended<F, T>(f: F) -> T
    where F: FnOnce() -> T
{
    let _disabled = SuspendInterrupts::new();
    f()
}

impl SuspendInterrupts {
    #[inline(always)]
    pub fn new() -> SuspendInterrupts {
        let ret = SuspendInterrupts(SREG::read());
        unsafe { asm!("cli") }
        ret
    }
}

impl Drop for SuspendInterrupts {
    #[inline(always)]
    fn drop(&mut self) {
        SREG::write(self.0);
    }
}
