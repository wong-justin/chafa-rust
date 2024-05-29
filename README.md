# chafa-rust (wip)

Rust bindings for [chafa](https://github.com/hpjansson/chafa)

## Demo

`examples/image2ansi.rs`

```rust
use chafa::{image2ansi, Config};

fn main() {
    let ansi_output = image2ansi("examples/test.png", Config{ 
        dst_width : 33,
        dst_height : 16,
    });
    println!("{}", ansi_output);
}
```

| Before                                       | After                                                               |
|----------------------------------------------|---------------------------------------------------------------------|
| ![original flowery image](examples/test.png) | ![flowery image displayed in terminal](examples/output_capture.png) |
|----------------------------------------------|---------------------------------------------------------------------|

Note: I'm new to both `chafa` and Rust's FFI binding process, so things may be a bit broken.

Looking for help: configuring builds + compilation, especially for Windows and Mac
