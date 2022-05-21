//! Re-exports commonly-used APIs that can be imported at once.

pub use crate::interrupt::with_irqs_disabled;
pub use crate::interrupt::with_irqs_suspended;
#[allow(deprecated)]
#[deprecated]
pub use crate::interrupt::without_interrupts;
