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

.PHONY: r
r:
	cargo build --release
	cp target/release/assemblers-solver.exe .

.PHONY: soft-clean
soft-clean:
	cargo clean -p $(CRATE_NAME)

.PHONY: clean
clean:
	cargo clean
