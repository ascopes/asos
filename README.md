# asos 

An OS built in Rust. Following https://os.phil-opp.com/minimal-rust-kernel/#building-our-kernel as a tutorial.

## Installing bootimage

To be able to make a bootable image that you can quickly run, invoke the following two commands from your shell first:

```bash
cargo install bootimage

rustup component add llvm-tools-preview
```

## Building for specific targets

To build for a specific target, run `cargo build --target {name}`. For example:

```bash
cargo build --target x86_64-asos.json
```

## Running in QEMU

You will need QEMU installed first.

Run `cargo run` to start the generated bootimage in a QEMU virtual machine.

## Defining targets

Targets must be placed in a file such as `x86_64-asos.json` in the root of this repository, and look like the following:

```js
{
  "llvm-target": "x86_64-unknown-none",
  "data-layout": "e-m:e-i64:64-f80:128-n8:16:32:64-S128",
  "arch": "x86_64",
  "target-endian": "little",
  "target-pointer-width": "64",
  "target-c-int-width": "32",
  "os": "none",
  "executables": true,
  "linker-flavor": "ld.lld",
  "linker": "rust-lld",
  "panic-strategy": "abort",
  "disable-redzone": true,
  "features": "-mmx,-sse,+soft-float"
}
```

There are a few things to note here.

1. `linker-flavor` is set to `ld.lld`, and `linker` is set to `rust-lld`. This is needed to link correctly.
2. `panic-strategy` is set to `abort`, which disables stack unwinding functionality.
3. `disable-redzone` is set to `true`. The System-V ABI allows functions to temporarily use the 128 bytes below
   the stack pointer without adjusting the pointer itself. We need to disable this to prevent stack corruption in
   the OS itself.
4. `features` disables `-mmx` and `-sse`. While SIMD extensions _can_ be faster, when context switching, the use of
   these registers will significantly slow down this process. The reason for this is that there would be more registers
   to repeatedly store in memory and then restore back into CPU registers each time.
5. `features` enables `soft-float`. This will force floating point operations on x86 to be emulated via software
   functions rather than using SIMD registers (see point #4).
   