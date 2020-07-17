// minimal example for the stm32-f401 (and the f4 series)
//! Prints "Hello, world!" on the host console using semihosting

#![no_main]
#![no_std]

extern crate panic_halt;

use stm32f4::stm32f401 as stm32;

use cortex_m::asm;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;


// Lab:
// - Create a mutable "global" static array.
// - Assign the values 0..10 to said array.
// - Print said array on one line instead of a loop.
static mut NUMBER_ARRAY: [u8; 10] = [0;10]; 

#[entry]
fn main() -> ! {
    hprintln!("Hello, world!").unwrap();
    unsafe {
        for x  in 0..10 as usize {
            NUMBER_ARRAY[x] = x as u8;
        }
        hprintln!("{:?}",NUMBER_ARRAY);
        }

    loop {
        asm::nop();
    }
}

// See RM0368 Reference Manual for details
// https://www.st.com/content/ccc/resource/technical/document/reference_manual/5d/b1/ef/b2/a1/66/40/80/DM00096844.pdf/files/DM00096844.pdf/jcr:content/translations/en.DM00096844.pdf
//
// Memory map is given in `memory.x`, in particular we define:
// 96k RAM starting at 0x2000_0000, and
// 256k Flash starting at 0x0800_0000
//
// Run in separate terminal:
// openocd -f openocd.cfg
//
// cargo run --example f401_minimal --features f4 --target thumbv7em-none-eabihf
//
// ...
// DefaultPreInit () at /home/pln/.cargo/git/checkouts/cortex-m-rt-073d0396a6df513c/8d26860/src/lib_thumb_rt.rs:571
// 571     pub unsafe extern "C" fn DefaultPreInit() {}
// (gdb)
//
// At this point, the progrom has been flashed onto the target (stm32f401/f411).
//
// (gdb) c
// Continuing.
//
// Breakpoint 1, main () at examples/f401_minimal.rs:14
// 14      #[entry]
//
// `main` is our "entry" point for the user application.
// It can be named anything by needs to annotated by #[entry].
// At this point global variables have been initiated.
//
// The `openocd.gdb` script defines the startup procedure, where we have set
// a breakpoint at the `main` symbol.
//
// Let's continue.
//
// (gdb) c
// Continuing.
// halted: PC: 0x08001206
//
// In the `openocd` terminal the `Hello world!` text should appear.
// The program is stuck in the infinite `loop {}`.
//
// If you press Ctrl-C, you will force the target (stm32fxx) to break.
//
// (gdb) c
// Continuing.
// halted: PC: 0x08001206
// ^C
// Program received signal SIGINT, Interrupt.
// f401_minimal::__cortex_m_rt_main () at examples/f401_minimal.rs:19
// 19          loop {
//
// (gdb) disassemble
// Dump of assembler code for function f401_minimal::__cortex_m_rt_main:
// 0x0800019a <+0>:     movw    r0, #5104       ; 0x13f0
// 0x0800019e <+4>:     movt    r0, #2048       ; 0x800
// 0x080001a2 <+8>:     movs    r1, #14
// 0x080001a4 <+10>:    bl      0x8000f86 <cortex_m_semihosting::export::hstdout_str>
// 0x080001a8 <+14>:    movw    r1, #5144       ; 0x1418
// 0x080001ac <+18>:    movt    r1, #2048       ; 0x800
// 0x080001b0 <+22>:    bl      0x80011fa <core::result::Result<T,E>::unwrap>
// 0x080001b4 <+26>:    b.n     0x80001b6 <f401_minimal::__cortex_m_rt_main+28>
// 0x080001b6 <+28>:    bl      0x80012b4 <__nop>
// => 0x080001ba <+32>:    b.n     0x80001b6 <f401_minimal::__cortex_m_rt_main+28>
//
// (In gdb you may use tab, for command completion, up arrow for previous command and
// shortcuts for just about anything, `c` for `continue` e.g.)
//
// 0x080001b6 <+28>:    bl      0x80012b4 <__nop>
// is a "branch and link call" to a function `__nop`, that simply does nothing (no operation).
// 0x080001ba <+32>:    b.n     0x80001b6 <f401_minimal::__cortex_m_rt_main+28>
// in a branch to the previous instruction, indeed an infinite loop right.
//
// Some basic Rust.
// Use https://www.rust-lang.org/learn and in particular https://doc.rust-lang.org/book/.
// There is even a book on embedded Rust available:
// https://rust-embedded.github.io/book/, it covers much more than we need here.
//
// Figure out a way to print the numbers 0..10 using a for loop.
//
// Figure out a way to store the numbers in 0..10 in a static (global) array using a loop.
//
// Print the resulting array (using a single println invocation, not a loop).
//
// (You may prototype the code directly on https://play.rust-lang.org/, and when it works
// back-port that into the minimal example, and check that it works the same)
//
// These two small exercises should get you warmed up.
//
// Some reflections:
// Why is does dealing with static variables require `unsafe`?
//
// This is the way!
