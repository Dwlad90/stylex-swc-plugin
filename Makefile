# StyleX SWC Plugin Makefile
# This Makefile provides convenient shortcuts for common development tasks
# in this monorepo containing both Rust and Node.js packages.

# Variables
PNPM := pnpm
CARGO := cargo
TURBO := $(PNPM) turbo

# Colors for output
YELLOW := \033[1;33m
GREEN := \033[1;32m
BLUE := \033[1;34m
NC := \033[0m # No Color

# Default target
.DEFAULT_GOAL := help

# Declare phony targets
.PHONY: help install clean build build-rust build-node dev test test-visual bench lint format typecheck docs setup prepare release publish check-deps

# Help target - shows available commands
help: ## Show this help message
	@echo "$(BLUE)StyleX SWC Plugin Development Commands$(NC)"
	@echo "======================================"
	@echo ""
	@echo "$(YELLOW)Setup Commands:$(NC)"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | grep -E "(install|setup|prepare)" | awk 'BEGIN {FS = ":.*?## "}; {printf "  $(GREEN)%-15s$(NC) %s\n", $$1, $$2}'
	@echo ""
	@echo "$(YELLOW)Build Commands:$(NC)"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | grep -E "(build|clean)" | awk 'BEGIN {FS = ":.*?## "}; {printf "  $(GREEN)%-15s$(NC) %s\n", $$1, $$2}'
	@echo ""
	@echo "$(YELLOW)Development Commands:$(NC)"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | grep -E "(dev|watch)" | awk 'BEGIN {FS = ":.*?## "}; {printf "  $(GREEN)%-15s$(NC) %s\n", $$1, $$2}'
	@echo ""
	@echo "$(YELLOW)Quality Commands:$(NC)"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | grep -E "(lint|format|typecheck|check)" | awk 'BEGIN {FS = ":.*?## "}; {printf "  $(GREEN)%-15s$(NC) %s\n", $$1, $$2}'
	@echo ""
	@echo "$(YELLOW)Test Commands:$(NC)"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | grep -E "(test|bench)" | awk 'BEGIN {FS = ":.*?## "}; {printf "  $(GREEN)%-15s$(NC) %s\n", $$1, $$2}'
	@echo ""
	@echo "$(YELLOW)Documentation & Release:$(NC)"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | grep -E "(docs|release|publish)" | awk 'BEGIN {FS = ":.*?## "}; {printf "  $(GREEN)%-15s$(NC) %s\n", $$1, $$2}'

# =============================================================================
# Setup Commands
# =============================================================================

install: ## Install all dependencies (both Node.js and Rust)
	@echo "$(YELLOW)Installing Node.js dependencies...$(NC)"
	$(PNPM) install
	@echo "$(YELLOW)Installing Rust toolchain...$(NC)"
	rustup show
	@echo "$(GREEN)All dependencies installed!$(NC)"

setup: install ## Full development environment setup
	@echo "$(YELLOW)Setting up development environment...$(NC)"
	$(PNPM) prepare
	@echo "$(GREEN)Development environment ready!$(NC)"

prepare: ## Prepare hooks and development tools
	$(PNPM) prepare

# =============================================================================
# Build Commands
# =============================================================================

clean: ## Clean all build artifacts
	@echo "$(YELLOW)Cleaning build artifacts...$(NC)"
	$(TURBO) clean
	$(CARGO) clean
	@echo "$(GREEN)Clean completed!$(NC)"

build: ## Build all packages (Node.js and Rust)
	@echo "$(YELLOW)Building all packages...$(NC)"
	$(TURBO) run build
	@echo "$(GREEN)Build completed!$(NC)"

build-rust: ## Build only Rust packages
	@echo "$(YELLOW)Building Rust packages...$(NC)"
	$(CARGO) build --workspace --release
	@echo "$(GREEN)Rust build completed!$(NC)"

build-node: ## Build only Node.js packages
	@echo "$(YELLOW)Building Node.js packages...$(NC)"
	$(TURBO) run build --filter="!@stylexswc/rs-compiler" --filter="!@stylexswc/test-parser"
	@echo "$(GREEN)Node.js build completed!$(NC)"

build-debug: ## Build Rust packages in debug mode
	@echo "$(YELLOW)Building Rust packages (debug)...$(NC)"
	$(CARGO) build --workspace
	@echo "$(GREEN)Debug build completed!$(NC)"

# =============================================================================
# Development Commands
# =============================================================================

dev: ## Start development servers
	@echo "$(YELLOW)Starting development servers...$(NC)"
	$(TURBO) dev

watch: ## Watch for changes and rebuild
	@echo "$(YELLOW)Starting watch mode...$(NC)"
	$(TURBO) dev

# =============================================================================
# Quality Commands
# =============================================================================

lint: ## Run linting on all packages
	@echo "$(YELLOW)Running linters...$(NC)"
	$(TURBO) lint --continue
	@echo "$(GREEN)Linting completed!$(NC)"

lint-check: ## Run linting with output to files
	@echo "$(YELLOW)Running lint checks...$(NC)"
	$(TURBO) run lint:check --continue
	@echo "$(GREEN)Lint checks completed!$(NC)"

format: ## Format all code (Prettier, Rust, TOML)
	@echo "$(YELLOW)Formatting code...$(NC)"
	$(PNPM) format
	@echo "$(YELLOW)Formatting Rust code...$(NC)"
	$(CARGO) fmt --all
	@echo "$(GREEN)Code formatting completed!$(NC)"

format-check: ## Check code formatting without making changes
	@echo "$(YELLOW)Checking code formatting...$(NC)"
	$(TURBO) run format:check --continue
	$(CARGO) fmt --all -- --check
	@echo "$(GREEN)Format check completed!$(NC)"

typecheck: ## Run TypeScript type checking
	@echo "$(YELLOW)Running type checks...$(NC)"
	$(TURBO) run typecheck --continue
	@echo "$(GREEN)Type checking completed!$(NC)"

check-deps: ## Check dependency versions across workspace
	@echo "$(YELLOW)Checking dependency versions...$(NC)"
	$(PNPM) syncpack list-mismatches
	@echo "$(GREEN)Dependency check completed!$(NC)"

# =============================================================================
# Test Commands
# =============================================================================

test: ## Run all tests
	@echo "$(YELLOW)Running tests...$(NC)"
	$(TURBO) run test --continue
	@echo "$(GREEN)Tests completed!$(NC)"

test-visual: ## Run visual regression tests
	@echo "$(YELLOW)Running visual tests...$(NC)"
	$(TURBO) run test:visual
	@echo "$(GREEN)Visual tests completed!$(NC)"

test-rust: ## Run only Rust tests
	@echo "$(YELLOW)Running Rust tests...$(NC)"
	$(CARGO) test --workspace
	@echo "$(GREEN)Rust tests completed!$(NC)"

bench: ## Run benchmarks
	@echo "$(YELLOW)Running benchmarks...$(NC)"
	$(TURBO) run bench
	@echo "$(GREEN)Benchmarks completed!$(NC)"

# =============================================================================
# Documentation & Release Commands
# =============================================================================

docs: ## Generate documentation
	@echo "$(YELLOW)Generating documentation...$(NC)"
	$(PNPM) docs
	@echo "$(GREEN)Documentation generated!$(NC)"

release: build ## Prepare packages for release
	@echo "$(YELLOW)Preparing release...$(NC)"
	@echo "$(GREEN)Release preparation completed!$(NC)"

publish: ## Publish packages to npm registry
	@echo "$(YELLOW)Publishing packages...$(NC)"
	@echo "$(BLUE)Note: Make sure you have proper npm credentials configured$(NC)"
	# Add specific publish commands here based on your release process
	@echo "$(GREEN)Publish completed!$(NC)"

# =============================================================================
# Utility Commands
# =============================================================================

cargo-check: ## Run cargo check on all Rust packages
	@echo "$(YELLOW)Running cargo check...$(NC)"
	$(CARGO) check --workspace --all-targets --all-features

clippy: ## Run clippy linter on Rust code
	@echo "$(YELLOW)Running clippy...$(NC)"
	$(CARGO) clippy --workspace --all-targets --all-features -- -D warnings

# Individual package commands
build-compiler: ## Build the Rust compiler package
	@echo "$(YELLOW)Building rs-compiler...$(NC)"
	cd crates/stylex-rs-compiler && $(PNPM) run build

build-unplugin: ## Build the unplugin package
	@echo "$(YELLOW)Building unplugin...$(NC)"
	cd packages/unplugin && $(PNPM) run build

build-nextjs: ## Build the Next.js plugin package
	@echo "$(YELLOW)Building nextjs-plugin...$(NC)"
	cd packages/nextjs-plugin && $(PNPM) run build

# Development shortcuts
quick-check: ## Quick development check (format, lint, typecheck)
	@echo "$(YELLOW)Running quick development checks...$(NC)"
	make format-check
	make lint
	make typecheck
	@echo "$(GREEN)Quick checks completed!$(NC)"

full-check: quick-check test ## Full development check including tests
	@echo "$(GREEN)Full development check completed!$(NC)"

# Show project info
info: ## Show project information
	@echo "$(BLUE)StyleX SWC Plugin Project Information$(NC)"
	@echo "====================================="
	@echo "Package Manager: $(shell $(PNPM) --version)"
	@echo "Node.js Version: $(shell node --version)"
	@echo "Rust Version: $(shell rustc --version)"
	@echo "Cargo Version: $(shell cargo --version)"
	@echo ""
	@echo "$(YELLOW)Workspace Packages:$(NC)"
	@$(PNPM) list --depth=0 2>/dev/null || echo "Run 'make install' first"
