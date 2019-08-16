CRATE_NAME:=assemblers-solver

.PHONY: default
default: check

.PHONY: all
all: soft-clean build check

.PHONY: build
build:
	cargo build

.PHONY: check
check: soft-clean
	cargo test
	cargo fmt -- --check
	cargo clippy -- -D warnings

.PHONY: release
release: check
	cargo build --release

.PHONY: soft-clean
soft-clean:
	cargo clean -p $(CRATE_NAME)

.PHONY: clean
clean:
	cargo clean
