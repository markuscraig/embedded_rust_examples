// do not link to the rust 'std' crate; instead
// link to the 'core' crate
#![no_std]

// do not use the main interface; using main with
// no_std requires rust nightly
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m::asm;
use cortex_m_rt::entry;

// entry-point of the program; attribute provided
// by the cortex-m-rt crate
#[entry]
fn main() -> ! { // mark main as never returning (at compile time)
    asm::nop(); // To not have main optimize to abort in release mode, remove when you add code

    loop {
        // your code goes here
    }
}