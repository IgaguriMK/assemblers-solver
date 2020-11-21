CRATE_NAME:=assemblers-solver

.PHONY: check
check: soft-clean
	cargo fmt -- --check
	cargo test
	cargo clippy -- -D warnings
	cargo audit
	cargo outdated --exit-code 1

.PHONY: soft-clean
soft-clean:
	cargo clean -p $(CRATE_NAME)
