dnl <!-- this template uses some m4 macros -->
changequote(`{{', `}}')dnl
dnl
# chafa-rust (wip)

Rust bindings for [chafa](https://github.com/hpjansson/chafa), a terminal graphics library:

> With chafa, you can now view very, very reasonable approximations of pictures and animations in the comfort of your favorite terminal emulator. The power of ANSI X3.64 compels you!

## Demo

Using a convenience function: `examples/image2ansi.rs`

```rust
include({{examples/image2ansi.rs}})dnl
```

| Before                                       | After                                                               |
|----------------------------------------------|---------------------------------------------------------------------|
| ![original flowery image](examples/test.png) | ![flowery image displayed in terminal](examples/output_capture.png) |

Or using the normal chafa API: `examples/demo.rs`

```rust
include({{examples/demo.rs}})dnl
```

## Build

- Uses `rustc --version` `1.65.0`

- Dependencies: `chafa` and `glib` must be installed. See the [chafa installation page](https://hpjansson.org/chafa/download/), the [compilation page](https://hpjansson.org/chafa/ref/chafa-building.html), and my notes at `chafa-sys/build.rs::main`.

- `git clone https://github.com/wong-justin/chafa-rust.git`

- `cd chafa-rust && cargo build`

- Confirm that the example works by running `cargo run --example image2ansi`

## Docs

See the [chafa C API](https://hpjansson.org/chafa/ref/index.html).

---

Note: I'm new to both `chafa` and Rust's FFI binding process, so things may be a bit broken.

Looking for help: configuring builds + compilation, especially for Windows and Mac
