.PHONY: help help-all build release install run outdated test lint check

.DEFAULT_GOAL := help

help:
	@echo "Usage: make [target]"
	@echo ""
	@echo "  check   - run lint + build + test"
	@echo "  run     - print help for the binary"
	@echo "  test    - run unit and integration tests"
	@echo ""
	@echo "Run 'make help-all' for the full list, including internal targets."

help-all:
	@echo "Public targets:"
	@echo "  check   - run lint + build + test"
	@echo "  build   - compile debug binary"
	@echo "  release - compile release binary + strip"
	@echo "  install - release + copy to ~/.local/bin"
	@echo "  run     - print help for the binary"
	@echo "  test    - run unit and integration tests"
	@echo ""
	@echo "Internal targets:"
	@echo "  lint    - run fmt check + clippy"
	@echo "  outdated - list outdated dependencies"

build:
	cargo build

release:
	cargo build --release
	strip target/release/elapsed 2>/dev/null || echo "strip not found, skipping"

install: release
	mkdir -p "$${HOME}/.local/bin"
	cp -f target/release/elapsed "$${HOME}/.local/bin/elapsed"
	@echo "Installed to $${HOME}/.local/bin/elapsed"

run:
	cargo run -- --help

outdated:
	cargo outdated -R

test:
	cargo test

lint:
	cargo fmt --all -- --check
	cargo clippy --all-targets --all-features -- -D warnings

check: lint
	cargo build
	cargo test
	@echo "ALL CHECKS PASSED"
