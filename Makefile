.PHONY: help build test clean install run check fmt clippy release all
.PHONY: version tag-release bump-patch bump-minor bump-major release-workflow

# Default target
.DEFAULT_GOAL := help

# Colors for output
YELLOW := \033[1;33m
GREEN := \033[1;32m
CYAN := \033[1;36m
RED := \033[1;31m
NC := \033[0m # No Color

# Version management
VERSION := $(shell grep '^version' crates/nara/Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')

##@ Help

help: ## Display this help message
	@echo "$(CYAN)Nara Language - Development Commands$(NC)"
	@echo ""
	@awk 'BEGIN {FS = ":.*##"; printf "Usage:\n  make $(YELLOW)<target>$(NC)\n"} /^[a-zA-Z_0-9-]+:.*?##/ { printf "  $(CYAN)%-20s$(NC) %s\n", $$1, $$2 } /^##@/ { printf "\n$(GREEN)%s$(NC)\n", substr($$0, 5) } ' $(MAKEFILE_LIST)

##@ Build

build: ## Build debug binaries
	cargo build

release: ## Build optimized release binaries
	cargo build --release

clean: ## Remove build artifacts
	cargo clean

install: ## Install nara-cli locally
	cargo install --path crates/nara-cli

##@ Development

test: ## Run all tests
	cargo test

check: ## Run cargo check
	cargo check --workspace

fmt: ## Format code with rustfmt
	cargo fmt --all

clippy: ## Run clippy linter
	cargo clippy --workspace --all-targets --all-features -- -D warnings

run: ## Run REPL
	cargo run -q --bin nara-cli

all: fmt clippy test build ## Run fmt, clippy, test, and build

##@ Version Management

version: ## Display current version
	@echo "Current version: $(VERSION)"

tag-release: ## Create git tags for current version
	@echo "Tagging release v$(VERSION)"
	@git tag -a "v$(VERSION)" -m "Release v$(VERSION)" || echo "Tag v$(VERSION) might already exist"
	@git tag -a "nara-v$(VERSION)" -m "nara library v$(VERSION)" || echo "Tag nara-v$(VERSION) might already exist"
	@git tag -a "nara-cli-v$(VERSION)" -m "nara-cli tool v$(VERSION)" || echo "Tag nara-cli-v$(VERSION) might already exist"
	@echo "Created tags: v$(VERSION), nara-v$(VERSION), nara-cli-v$(VERSION)"
	@echo "Push tags with: git push origin --tags"

bump-patch: ## Bump patch version (0.1.0 -> 0.1.1)
	@echo "Bumping patch version..."
	@cd crates/nara && cargo set-version --bump patch
	@cd crates/nara-cli && cargo set-version --bump patch
	@echo "Version bumped. Run 'make version' to see new version."

bump-minor: ## Bump minor version (0.1.0 -> 0.2.0)
	@echo "Bumping minor version..."
	@cd crates/nara && cargo set-version --bump minor
	@cd crates/nara-cli && cargo set-version --bump minor
	@echo "Version bumped. Run 'make version' to see new version."

bump-major: ## Bump major version (0.1.0 -> 1.0.0)
	@echo "Bumping major version..."
	@cd crates/nara && cargo set-version --bump major
	@cd crates/nara-cli && cargo set-version --bump major
	@echo "Version bumped. Run 'make version' to see new version."

release-workflow: ## Run full release workflow (fmt, clippy, test, build)
	@echo "Running release workflow..."
	@make all
	@make release
	@echo ""
	@echo "All checks passed! Ready to tag release."
	@echo "Run 'make tag-release' to create git tags."

