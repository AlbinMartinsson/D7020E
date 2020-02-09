# klee-examples

This repo contains a set of usage examples for `klee-sys` low-level KLEE bindings. For more information on internal design behind see the [klee-sys](https://gitlab.henriktjader.com/pln/klee-sys) repo.

See section `Cargo.toml` for detailed information on features introduced.

### General dependencies

- LLVM toolchain tested with (9.0.1)
- rustup tested with 1.40.0 (73528e339 2019-12-16)
- klee tested with KLEE 2.1-pre (https://klee.github.io)

- cargo-klee (installed from git)

---

## Basic test examples

- `paths.rs`

    This example showcase the different path termination conditions possible and their effect to KLEE test case generation.

- `assume_assert.rs`

    This example showcase contract based verification, and the possibilities to extract proofs.

- `struct.rs`

    This example show the case of partially symbolic structures.

---

## Hardware oriented test examples

- `vcell_test.rs`

    Simple test to showcase low-level [vcell](https://github.com/perlindgren/vcell) access. `vcell` underlies all machine generated hardware accesses in the Rust embedded ecosystem.

    The `klee-analysis` feature replaces `read` operations to memory by symbolic values (using `klee-sys`). `write` operations are suppressed as for analysis we are not interested in the side effects.

- `register_test.rs`

    Simple test to showcase the use of the `volatile-register` abstraction. `volitile-register` builds on `vcell` and is used by both hand written and machine generated hardware accesses in the Rust embedded ecosystem.

    This example also showcase the `gdb` replay functionality.

    TODO: perhaps better to put the `gdb` replay in the Basic test examples, 
    as replay is not tied to `volatile-register`.

- `cortex-m-test1.rs`

   Simple test to showcase the [cortex-m](https://github.com/perlindgren/vcell) abstraction of ARM-core peripherals ARM thumb assembly instructions and ARM thumb CPU registers. `cortex-m` uses the `volatile-register` abstraction for ARM-core peripherals. The `klee-analysis` feature replaces read operations on CPU registers by symbolic data and suppresses write operations as for analysis we are not interested in the side effects.

   Moreover the example showcase the discrepancies between Rust source code paths and paths in the generated (semantically equivalent) LLVM-IR.  

   TODO: perhaps the latter part should be moved to Basic test examples as it is not `cortex-m` specific.

- `cortex-m-test-nightly.rs`

    This example showcase how compiler "intrinsics" can be safely adopted by proving the absence of errors.

    TODO: perhaps this part should also be moved to Basic test examples as it is not `cortex-m` specific.

---

## Testing on hardware

### Additional Dependencies:

- `stm32401` Nucleo64 board or similar with recent `stlink` firmware (tested with latest firmware as of 2020-01-10).
- `openocd` (tested with version 0.10.0)
- `arm-none-eabi-gdb` (tested with version 0.8.3)
- llvm target `thumbv7em-none-eabihf` 
  - `> rustup show`, to show current Rust tool-chain and installed targets.
  - `> rustup target add <target>`, to add target, e.g., `thumbv7em-none-eab¡hf`.
- [cargo bin-utils](https://github.com/rust-embedded/cargo-binutils) (tested with version 0.1.7)
  
### Examples

- `f401_minimal.rs`

This example showcase the execution of a minimal "Hello World!" application on the stm32f401 (and similar targets from the f4).

- `f401_minimal2.rs`

This example showcase cycle accurate and non-intrusive execution time measurements. It also covers, debug vs. release mode optimization and the effect of the `inline-asm` feauture.

- `f401_probe.rs`

A continuation of `f401_minimal2.rs`, showcasing the ultimate degree of automation possible, in an all Rust profiling setting. Further information is found in the `runner` crate.

---

### Disclaimer

This project is in early development, thus expect bugs and shortcomings, no API stability offered or guaranteed. It is however well suited for experimentation and all implemented features have been successfully tested.

---

## Licencse

Copyright Per Lindgren.

All rights reserved, use restricted for non-commercial purpose.
