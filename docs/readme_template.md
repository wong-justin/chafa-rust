dnl <!-- this template uses some m4 macros -->
changequote(`{{', `}}')dnl
dnl
# chafa-rust (wip)

Rust bindings for [chafa](https://github.com/hpjansson/chafa), a terminal graphics library:

> With chafa, you can now view very, very reasonable approximations of pictures and animations in the comfort of your favorite terminal emulator. The power of ANSI X3.64 compels you!

## Demo

Using the chafa API: `examples/demo.rs`

```rust
include({{examples/demo.rs}})dnl
```

Or using a convenience function: `examples/image2ansi.rs`

```rust
include({{examples/image2ansi.rs}})dnl
```

| Before                                       | After                                                               |
|----------------------------------------------|---------------------------------------------------------------------|
| ![original flowery image](examples/test.png) | ![flowery image displayed in terminal](examples/output_capture.png) |

## Docs

See the [chafa C API](https://hpjansson.org/chafa/ref/index.html).

## Usage

Install `chafa` with its dependency [`glib`](https://docs.gtk.org/glib/), either from your package manager or from source.
Then put this crate in your Rust project.

Typical usage:

```toml
[dependencies]
chafa = { features = ["link-dynamic"] }
``` 

<!-- for an example of a rust project using chafa, see [vic]. -->

### 1) Give library locations if necessary

By default, [pkg-config](https://people.freedesktop.org/~dbn/pkg-config-guide.html) tries to find `chafa` and its dependencies.
If libraries are not found, you can provide library names and directories with `$RUSTFLAGS`.

Example: `RUSTFLAGS="-L /usr/lib -l chafa -l glib" cargo build`

This may be useful when:

- libraries are located at non-standard locations
- libraries are not built with pkg-config metadata
- building in environments with unique linking needs, like when compiling with musl libc
- building on Windows? or environments that do not support pkg-config

<!-- metion .a or .so files? -->

### 2) Choose dynamic or static linking

This crate has two cargo build features: `["link-dynamic"]` or `["link-static"]`.
You must choose one.

Static builds have been trickier in my experience, mainly due to `glib` compilation errors.
I was able to make a statically linked build in an Alpine [Dockerfile](https://github.com/wong-justin/vic/blob/main/Dockerfile).
Other people have statically linked `chafa` & `glib` in other environments -- see [vic issue #1](https://github.com/wong-justin/vic/issues/1) to explore those options.

### Extra

You can enable the optional `["image"]` feature to access `chafa::extra::image2ansi`, which might be useful for casual usage.

## Status

This crate does not:

- build `chafa` or `glib` from source (yet).
- test builds on Windows or macOS (yet).

If you need those things, I recommend modifying `chafa-sys/build.rs` or writing your own containerized build script.
If you can vendor `chafa` and `glib` and build them from source in `chafa-sys/build.rs`, or if you have changes that make the build work on Windows or macOS, or if you have usability suggestions in general, your contribution would be appreciated.

`src/lib.rs` currently covers a minimal but usable amount of chafa functions.
Parity with [chafa's Python bindings](https://github.com/GuardKenzie/chafa.py) is a stretch goal.

## Licenses

`chafa` uses LGPL-3.0-or-later.

`glib` uses LGPL-2.1-or-later.
