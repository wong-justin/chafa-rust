dnl <!-- this template uses some m4 macros -->
changequote(`{{', `}}')dnl
dnl
# chafa-rust (wip)

Rust bindings for [chafa](https://github.com/hpjansson/chafa), a library for displaying images in the terminal.

## Demo

`examples/image2ansi.rs`

```rust
include({{examples/image2ansi.rs}})dnl
```

| Before                                       | After                                                               |
|----------------------------------------------|---------------------------------------------------------------------|
| ![original flowery image](examples/test.png) | ![flowery image displayed in terminal](examples/output_capture.png) |

## Build

- Dependencies: `chafa` and `glib` must be installed. See the [chafa installation page](https://hpjansson.org/chafa/download/), the [compilation page](https://hpjansson.org/chafa/ref/chafa-building.html), and my notes at `chafa-sys/build.rs::main`.

- `git clone https://github.com/wong-justin/chafa-rust.git`

- `cd chafa-rust && cargo build`

- Confirm that the example works by running `cargo run --example image2ansi`

---

Note: I'm new to both `chafa` and Rust's FFI binding process, so things may be a bit broken.

Looking for help: configuring builds + compilation, especially for Windows and Mac
