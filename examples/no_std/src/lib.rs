// Disable linking to std.
#![no_std]
// Use default alloc error handler, i.e. to panic, and enable core intrinsics.
#![feature(default_alloc_error_handler, core_intrinsics)]

// Abort when panicking.
#[panic_handler]
pub fn panic(_: &core::panic::PanicInfo) -> ! {
    core::intrinsics::abort();
}

// Use WeeAlloc as our global heap allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

//============================
// Scrypto code starts here //
//============================

use scrypto::prelude::*;

blueprint! {
    struct Greeting {
        count: u32
    }

    impl Greeting {
        pub fn new() -> Address {
            let component = Self {
                count: 0
            }.instantiate();

            debug!("New component: {}", component);
            component
        }

        pub fn say_hello(&mut self) {
            info!("Hello, visitor #{}.", self.count);
            self.count += 1;
        }
    }
}