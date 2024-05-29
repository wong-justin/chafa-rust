use std::env;
use std::path::PathBuf;

// use bindgen; 
// good: bindgen automates binding creation from C headers
// bad: bindgen requires libclang, which I guess is supposed to come with llvm as well 
// (https://rust-lang.github.io/rust-bindgen/requirements.html)


// also note: building chafa requires automake and libtool (my machine doesnt have libtool yet)
// (https://github.com/hpjansson/chafa?tab=readme-ov-file#installing)
// chafa must be installed in apt install style so that there's include/ files 
// since the include/ files are necessary for C tools for binding
// needs glib still
// chafa build step also complained about libdevtool2 or something like that, but i ignored by using the flag --without-tools

// but i can maybe compile from tarball
// (https://hpjansson.org/chafa/releases/chafa-1.14.0.tar.xz)
// does tarball compilation require the same tooling as compiling from git source?
// i still need glib tho


// helpful reference about linking object files and libraries:
// https://stackoverflow.com/a/29728671


fn main() {


    // STEPS TO BUILD A RUST LIB WITH C BINDINGS / WRAP A C LIBRARY
    //
    // 0) the C library needs to be installed, or built and installed.
    //    apt install chafa wasn't available on my machine, so i had to clone and build chafa.
    //    chafa build instructions were a bit confusing for me, as most C libraries are for me.
    //    i needed to install glib, libdevtools, and a couple other things.
    //    although the author said: "You could build it --without-tools, 
    //    then you don't need any loaders. The deps will then be glib-2.0 and freetype."
    //    finally, chafa/autogen.sh, then make, then make install, worked
    //
    // 1) a header file must exist in this rust project directory root.
    //    probs best simply call it wrapper.h, with contents:
    //    #include <chafa.h>
    //
    //    (https://rust-lang.github.io/rust-bindgen/tutorial-2.html)
    //
    // 2) in this build script, we need to tell the build tools where to find the libraries.
    //    there's a few ways to declare these library paths:
    //
    //    a) write the include/ path args by hand, eg:
    //       .clang_arg("-I/usr/local/include/chafa")
    //       ...
    //    
    //    b) use the tool pkg-config to find these paths automatically.
    //       on the command-line, that looks like:
    //       pkg-config --cflags chafa
    //       which outputs the string:
    //       -I/usr/local/include/chafa -I/usr/local/lib...
    //
    //    c) I couldn't get this to work, but using either
    //       println!("cargo:rustc-link-search=/path/to/lib")
    //       or
    //       println!("cargo:rustc-link-lib=chafa")
    //       is supposed tell cargo about library locations
    //
    //    (https://rust-lang.github.io/rust-bindgen/tutorial-3.html)
    //
    // 3) create src/lib.rs with contents:
    //    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
    //
    //    also adding some #![allow(...)] directives to surpress warnings about C-syntax.
    //
    //    then cargo build should work. bindgen will create a bindings.rs file somewhere deep in target/
    //
    // 4) verify that the layout, size, and alignment of FFI structs matches 
    //    what bindgen thinks they should be, by running cargo test
    //
    //    (https://rust-lang.github.io/rust-bindgen/tutorial-4.html)
    //
    // 4.5) troubleshoot why some functions are an undefined reference?
    //      (error while loading shared libraries: no such file or directory libchafa.so.0)
    //      i guess the linking tools weren't finding /usr/local/lib/libchafa.so.0
    //
    //      EDIT - my solution was running the simple command `ldconfig`,
    //      which updates the cache for shared libraries.
    //      apparently it was not updated since i installed chafa.
    //
    // 5) create safe function wrappers around extern functions
    //    using Rust idioms and exposing a safe, high-level interface
    //
    //    (https://doc.rust-lang.org/nomicon/ffi.html)
    //    (https://medium.com/dwelo-r-d/using-c-libraries-in-rust-13961948c72a)
    //
    // 6) consider whitelisting only necessary functions (or blacklisting unused functions)
    //    to reduce the size of the bindings file
    //
    //    (https://rust-lang.github.io/rust-bindgen/allowlisting.html)



    // println!("cargo:rustc-flags=-l chafa -L /usr/local/lib/chafa"); // didn't help me
    // println!("cargo:rustc-link-lib=glib"); // didn't help me
    // println!("cargo:rustc-link-search=/usr/local/lib"); // didn't help me
    // env::set_var("LD_LIBRARY_PATH", "/usr/local/lib"); // didn't help me
    // env::set_var("PKG_CONFIG_ALLOW_SYSTEM_CFLAGS", "1"); // didn't help me

    let library = pkg_config::probe_library("chafa").expect("Unable to probe chafa library");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_args(library.include_paths.iter().map(|path| format!("-I{}", path.to_string_lossy())))
        // aka:
        // .clang_arg("-I/usr/local/include/chafa")
        // .clang_arg("-I/usr/local/lib/chafa/include")
        // .clang_arg("-I/usr/include/glib-2.0")
        // .clang_arg("-I/usr/lib/x86_64-linux-gnu/glib-2.0/include")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings")
}
