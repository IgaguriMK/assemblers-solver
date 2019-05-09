CRATE_NAME:=assemblers-solver

.PHONY: all
all: soft-clean build check

.PHONY: build
build:
	cargo build

.PHONY: check
check: soft-clean
	cargo clippy -- -D warnings
	cargo test

.PHONY: release
release: clean build check r

.PHONY: soft-clean
soft-clean:
	cargo clean -p $(CRATE_NAME)

.PHONY: clean
clean:
	cargo clean
