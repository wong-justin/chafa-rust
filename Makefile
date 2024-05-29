
# current build process is described in ./chafa-sys/build.rs
# build depends on having chafa installed, which means having glib-2.0 installed
# TODO: formalize dependencies for libchafa.so.0 and glib-2.0
# maybe a github action like:
# apt install glib-2.0 && \
# git clone github/chafa/v1.14.0 && \
# cp /usr/lib/.../libchafa.so.0 ./
#
# still not sure how to build for windows or mac,
# regarding signed libraries and .dlls
#
# see python bindings CI/CD for inspiration:
# https://github.com/GuardKenzie/chafa.py/blob/main/pyproject.toml#L46-L58
# they have machines build wheel artifacts for each platform and distribute those on pypi
#
# OR
#
# i could manually create these libraries, both the .so for linux/mac and the .dlls for windows,
# put them in this directory like in ./vendor/lib/ or ./include/,
# and in the build script, match target_and_arch to the correct platform's library files
# like this guy did: https://kellnr.io/blog/cross-plat-native-lib
#
# and maybe even using [cfg(target_os="windows", target_arch="x86")] 
# instead of match (os.as_str(), arch.as_str())
# https://doc.rust-lang.org/reference/conditional-compilation.html
#
# OR
#
# maybe build chafa from src, which would probably still mean installing glib-2.0,
# but then all the compilation and object files would happen in build.rs,
# like so: https://rust-lang.github.io/rust-bindgen/non-system-libraries.html


# note: the raw bindgen bindings and compilation steps are in a nested cargo project called chafa-sys/
# and the Rust API wrapper around those bindings is in this outer project root.
# this separation makes compile times faster,
# and that's the convention for rust binding projects.

build: chafa-sys/* src/* Cargo.toml /usr/local/include/chafa /usr/include/glib-2.0
	cargo build

test:
	cargo test

dev:
	find *.rs **/*.rs | entr -cs 'cargo run --example image2ansi'
