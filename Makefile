EXAMPLES = demo

.PHONY: build test run fmt clippy docs docs-serve clean example $(addprefix example_, $(EXAMPLES))

build:
	cargo build

test:
	cargo test

run:
	cargo run

fmt:
	cargo fmt

clippy:
	cargo clippy -- -D warnings

docs:
	mdbook build docs

docs-serve:
	mdbook serve docs --open

clean:
	cargo clean
	rm -rf docs/book

$(addprefix example_, $(EXAMPLES)): example_%:
	cargo run --example $*

example: $(addprefix example_, $(EXAMPLES))
