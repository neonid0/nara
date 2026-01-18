.PHONY: help build test clean install run check fmt clippy release all
.PHONY: version tag-release bump-patch bump-minor bump-major release-workflow

# Version management
VERSION := $(shell grep '^version' crates/nara/Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')

help:
	@echo "Nara Language - Development Commands"
	@echo ""
	@echo "Build commands:"
	@echo "  make build      - Build debug binaries"
	@echo "  make release    - Build optimized release binaries"
	@echo "  make test       - Run all tests"
	@echo "  make check      - Run cargo check"
	@echo "  make fmt        - Format code with rustfmt"
	@echo "  make clippy     - Run clippy linter"
	@echo "  make clean      - Remove build artifacts"
	@echo "  make install    - Install nara-cli locally"
	@echo "  make run        - Run REPL"
	@echo "  make all        - fmt + clippy + test + build"
	@echo ""
	@echo "Version management:"
	@echo "  make version         - Display current version"
	@echo "  make tag-release     - Create git tags for current version"
	@echo "  make bump-patch      - Bump patch version (0.1.0 -> 0.1.1)"
	@echo "  make bump-minor      - Bump minor version (0.1.0 -> 0.2.0)"
	@echo "  make bump-major      - Bump major version (0.1.0 -> 1.0.0)"
	@echo "  make release-workflow- Run full release workflow (fmt, clippy, test, build)"

build:
	cargo build

release:
	cargo build --release

test:
	cargo test

check:
	cargo check --workspace

fmt:
	cargo fmt --all

clippy:
	cargo clippy --workspace --all-targets --all-features -- -D warnings

clean:
	cargo clean

install:
	cargo install --path crates/nara-cli

run:
	cargo run -q --bin nara-cli

all: fmt clippy test build

version:
	@echo "Current version: $(VERSION)"

tag-release:
	@echo "Tagging release v$(VERSION)"
	@git tag -a "v$(VERSION)" -m "Release v$(VERSION)" || echo "Tag v$(VERSION) might already exist"
	@git tag -a "nara-v$(VERSION)" -m "nara library v$(VERSION)" || echo "Tag nara-v$(VERSION) might already exist"
	@git tag -a "nara-cli-v$(VERSION)" -m "nara-cli tool v$(VERSION)" || echo "Tag nara-cli-v$(VERSION) might already exist"
	@echo "Created tags: v$(VERSION), nara-v$(VERSION), nara-cli-v$(VERSION)"
	@echo "Push tags with: git push origin --tags"

bump-patch:
	@echo "Bumping patch version..."
	@cd crates/nara && cargo set-version --bump patch
	@cd crates/nara-cli && cargo set-version --bump patch
	@echo "Version bumped. Run 'make version' to see new version."

bump-minor:
	@echo "Bumping minor version..."
	@cd crates/nara && cargo set-version --bump minor
	@cd crates/nara-cli && cargo set-version --bump minor
	@echo "Version bumped. Run 'make version' to see new version."

bump-major:
	@echo "Bumping major version..."
	@cd crates/nara && cargo set-version --bump major
	@cd crates/nara-cli && cargo set-version --bump major
	@echo "Version bumped. Run 'make version' to see new version."

release-workflow:
	@echo "Running release workflow..."
	@make all
	@make release
	@echo ""
	@echo "All checks passed! Ready to tag release."
	@echo "Run 'make tag-release' to create git tags."

.DEFAULT_GOAL := help
