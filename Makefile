# still not sure how to build for windows or mac,
# regarding signed libraries and .dlls
#
# see python bindings CI/CD for inspiration:
# https://github.com/GuardKenzie/chafa.py/blob/main/pyproject.toml
# they have machines build wheel artifacts for each platform and distribute those on pypi
#
# or i could build chafa+glib, both the .so for linux/mac and the .dlls for windows,
# put them in this directory like in ./vendor/lib/ or ./include/,
# and in the build script, match target_and_arch to the correct platform's library files
# like this guy did: 
# https://kellnr.io/blog/cross-plat-native-lib
#
# and maybe even using [cfg(target_os="windows", target_arch="x86")] 
# instead of match (os.as_str(), arch.as_str())
# https://doc.rust-lang.org/reference/conditional-compilation.html
#
.PHONY: build
build: chafa-sys/* src/* Cargo.toml
	cargo build --features link-dynamic
	make docs

.PHONY: test
test:
	cargo test --features link-dynamic,image

.PHONY: dev
dev:
	@# some dev dependencies that i find useful:
	@# - entr, for livereloading
	@# - m4, for text file templating
	find *.rs **/*.rs | entr -cs 'cargo run --example image2ansi --features link-dynamic,image'

.PHONY: docs
docs: src/* docs/readme_template.md examples/image2ansi.rs
	@# m4 injects src code into readme so it automatically stays updated
	@# consider also making rustdocs: # cargo doc --no-deps --open
	m4 -I. docs/readme_template.md > README.md
