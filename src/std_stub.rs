//! Stub methods that `libstd` normally defines.

// These do not need to be in a module, but we group them here for clarity.
pub mod std {
    #[lang = "eh_personality"]
    #[no_mangle]
    pub unsafe extern "C" fn rust_eh_personality(_state: (), _exception_object: *mut (), _context: *mut ()) -> () {
    }

    #[panic_handler]
    fn panic(_info: &::core::panic::PanicInfo) -> ! {
        loop {}
    }
}

