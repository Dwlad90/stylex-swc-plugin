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
.PHONY: help install clean build build-rust build-node dev test test-visual bench lint format typecheck docs setup prepare release publish check-deps \
	apps-build apps-dev apps-clean apps-serve-common app-nextjs-dev app-nextjs-build app-nextjs-serve app-vite-dev app-vite-build app-vite-serve app-webpack-dev app-webpack-build app-rollup-dev app-rollup-build \
	packages-build packages-lint packages-test packages-typecheck packages-clean crates-build crates-format crates-lint crates-clean crates-docs \
	pkg-unplugin-build pkg-unplugin-lint pkg-unplugin-test pkg-unplugin-typecheck pkg-unplugin-clean \
	pkg-nextjs-build pkg-nextjs-lint pkg-nextjs-test pkg-nextjs-typecheck pkg-nextjs-clean \
	pkg-webpack-build pkg-webpack-lint pkg-webpack-test pkg-webpack-typecheck pkg-webpack-clean \
	pkg-rollup-build pkg-rollup-lint pkg-rollup-test pkg-rollup-typecheck pkg-rollup-clean \
	pkg-postcss-build pkg-postcss-lint pkg-postcss-test pkg-postcss-typecheck pkg-postcss-clean \
	pkg-jest-build pkg-jest-lint pkg-jest-test pkg-jest-typecheck pkg-jest-clean \
	pkg-design-build pkg-design-lint pkg-design-test pkg-design-typecheck pkg-design-clean \
	pkg-playwright-build pkg-playwright-lint pkg-playwright-test pkg-playwright-typecheck pkg-playwright-clean \
	pkg-eslint-build pkg-eslint-lint pkg-eslint-test pkg-eslint-typecheck pkg-eslint-clean \
	pkg-typescript-build pkg-typescript-lint pkg-typescript-test pkg-typescript-typecheck pkg-typescript-clean \
	crate-compiler-build crate-compiler-format crate-compiler-lint crate-compiler-clean crate-compiler-docs \
	crate-shared-build crate-shared-format crate-shared-lint crate-shared-clean crate-shared-docs \
	crate-resolver-build crate-resolver-format crate-resolver-lint crate-resolver-clean crate-resolver-docs \
	crate-parser-build crate-parser-format crate-parser-lint crate-parser-clean crate-parser-docs

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
	@echo "$(YELLOW)App Commands:$(NC)"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | grep -E "(app|apps)" | awk 'BEGIN {FS = ":.*?## "}; {printf "  $(GREEN)%-15s$(NC) %s\n", $$1, $$2}'
	@echo ""
	@echo "$(YELLOW)Package Commands:$(NC)"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | grep -E "(pkg|packages|crate|crates)" | awk 'BEGIN {FS = ":.*?## "}; {printf "  $(GREEN)%-15s$(NC) %s\n", $$1, $$2}' | head -20
	@echo "  $(BLUE)...and many more individual package commands$(NC)"
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
	$(TURBO) --filter "./crates/*" test
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

# =============================================================================
# Package Commands
# =============================================================================

# Bulk package operations
packages-build: ## Build all Node.js packages
	@echo "$(YELLOW)Building all Node.js packages...$(NC)"
	$(TURBO) run build --filter="./packages/*"
	@echo "$(GREEN)All Node.js packages built successfully!$(NC)"

packages-lint: ## Lint all Node.js packages
	@echo "$(YELLOW)Linting all Node.js packages...$(NC)"
	$(TURBO) run lint --filter="./packages/*" --continue
	@echo "$(GREEN)All Node.js packages linted successfully!$(NC)"

packages-test: ## Test all Node.js packages
	@echo "$(YELLOW)Testing all Node.js packages...$(NC)"
	$(TURBO) run test --filter="./packages/*" --continue
	@echo "$(GREEN)All Node.js package tests completed!$(NC)"

packages-typecheck: ## Typecheck all Node.js packages
	@echo "$(YELLOW)Typechecking all Node.js packages...$(NC)"
	$(TURBO) run typecheck --filter="./packages/*" --continue
	@echo "$(GREEN)All Node.js packages typechecked successfully!$(NC)"

packages-clean: ## Clean all Node.js packages
	@echo "$(YELLOW)Cleaning all Node.js packages...$(NC)"
	$(TURBO) run clean --filter="./packages/*"
	@echo "$(GREEN)All Node.js packages cleaned successfully!$(NC)"

# Bulk crate operations
crates-build: ## Build all Rust crates
	@echo "$(YELLOW)Building all Rust crates...$(NC)"
	$(CARGO) build --workspace --release
	@echo "$(GREEN)All Rust crates built successfully!$(NC)"

crates-format: ## Format all Rust crates
	@echo "$(YELLOW)Formatting all Rust crates...$(NC)"
	$(CARGO) fmt --all
	@echo "$(GREEN)All Rust crates formatted successfully!$(NC)"

crates-lint: ## Lint all Rust crates
	@echo "$(YELLOW)Linting all Rust crates...$(NC)"
	$(CARGO) clippy --workspace --all-targets --all-features -- -D warnings
	@echo "$(GREEN)All Rust crates linted successfully!$(NC)"

crates-clean: ## Clean all Rust crates
	@echo "$(YELLOW)Cleaning all Rust crates...$(NC)"
	$(CARGO) clean
	@echo "$(GREEN)All Rust crates cleaned successfully!$(NC)"

crates-docs: ## Generate docs for all Rust crates
	@echo "$(YELLOW)Generating docs for all Rust crates...$(NC)"
	$(CARGO) doc --workspace --no-deps
	@echo "$(GREEN)All Rust crate docs generated successfully!$(NC)"

# Individual Node.js package commands
# Unplugin package
pkg-unplugin-build: ## Build unplugin package
	@echo "$(YELLOW)Building unplugin package...$(NC)"
	cd packages/unplugin && $(PNPM) run build

pkg-unplugin-lint: ## Lint unplugin package
	@echo "$(YELLOW)Linting unplugin package...$(NC)"
	cd packages/unplugin && $(PNPM) run lint

pkg-unplugin-test: ## Test unplugin package
	@echo "$(YELLOW)Testing unplugin package...$(NC)"
	cd packages/unplugin && $(PNPM) run test

pkg-unplugin-typecheck: ## Typecheck unplugin package
	@echo "$(YELLOW)Typechecking unplugin package...$(NC)"
	cd packages/unplugin && $(PNPM) run typecheck

pkg-unplugin-clean: ## Clean unplugin package
	@echo "$(YELLOW)Cleaning unplugin package...$(NC)"
	cd packages/unplugin && $(PNPM) run clean

# Next.js plugin package
pkg-nextjs-build: ## Build Next.js plugin package
	@echo "$(YELLOW)Building nextjs-plugin package...$(NC)"
	cd packages/nextjs-plugin && $(PNPM) run build

pkg-nextjs-lint: ## Lint Next.js plugin package
	@echo "$(YELLOW)Linting nextjs-plugin package...$(NC)"
	cd packages/nextjs-plugin && $(PNPM) run lint

pkg-nextjs-test: ## Test Next.js plugin package
	@echo "$(YELLOW)Testing nextjs-plugin package...$(NC)"
	cd packages/nextjs-plugin && $(PNPM) run test

pkg-nextjs-typecheck: ## Typecheck Next.js plugin package
	@echo "$(YELLOW)Typechecking nextjs-plugin package...$(NC)"
	cd packages/nextjs-plugin && $(PNPM) run typecheck

pkg-nextjs-clean: ## Clean Next.js plugin package
	@echo "$(YELLOW)Cleaning nextjs-plugin package...$(NC)"
	cd packages/nextjs-plugin && $(PNPM) run clean

# Webpack plugin package
pkg-webpack-build: ## Build webpack plugin package
	@echo "$(YELLOW)Building webpack-plugin package...$(NC)"
	cd packages/webpack-plugin && $(PNPM) run build

pkg-webpack-lint: ## Lint webpack plugin package
	@echo "$(YELLOW)Linting webpack-plugin package...$(NC)"
	cd packages/webpack-plugin && $(PNPM) run lint

pkg-webpack-test: ## Test webpack plugin package
	@echo "$(YELLOW)Testing webpack-plugin package...$(NC)"
	cd packages/webpack-plugin && $(PNPM) run test

pkg-webpack-typecheck: ## Typecheck webpack plugin package
	@echo "$(YELLOW)Typechecking webpack-plugin package...$(NC)"
	cd packages/webpack-plugin && $(PNPM) run typecheck

pkg-webpack-clean: ## Clean webpack plugin package
	@echo "$(YELLOW)Cleaning webpack-plugin package...$(NC)"
	cd packages/webpack-plugin && $(PNPM) run clean

# Rollup plugin package
pkg-rollup-build: ## Build rollup plugin package
	@echo "$(YELLOW)Building rollup-plugin package...$(NC)"
	cd packages/rollup-plugin && $(PNPM) run build

pkg-rollup-lint: ## Lint rollup plugin package
	@echo "$(YELLOW)Linting rollup-plugin package...$(NC)"
	cd packages/rollup-plugin && $(PNPM) run lint

pkg-rollup-test: ## Test rollup plugin package
	@echo "$(YELLOW)Testing rollup-plugin package...$(NC)"
	cd packages/rollup-plugin && $(PNPM) run test

pkg-rollup-typecheck: ## Typecheck rollup plugin package
	@echo "$(YELLOW)Typechecking rollup-plugin package...$(NC)"
	cd packages/rollup-plugin && $(PNPM) run typecheck

pkg-rollup-clean: ## Clean rollup plugin package
	@echo "$(YELLOW)Cleaning rollup-plugin package...$(NC)"
	cd packages/rollup-plugin && $(PNPM) run clean

# PostCSS plugin package
pkg-postcss-build: ## Build PostCSS plugin package
	@echo "$(YELLOW)Building postcss-plugin package...$(NC)"
	cd packages/postcss-plugin && $(PNPM) run build

pkg-postcss-lint: ## Lint PostCSS plugin package
	@echo "$(YELLOW)Linting postcss-plugin package...$(NC)"
	cd packages/postcss-plugin && $(PNPM) run lint

pkg-postcss-test: ## Test PostCSS plugin package
	@echo "$(YELLOW)Testing postcss-plugin package...$(NC)"
	cd packages/postcss-plugin && $(PNPM) run test

pkg-postcss-typecheck: ## Typecheck PostCSS plugin package
	@echo "$(YELLOW)Typechecking postcss-plugin package...$(NC)"
	cd packages/postcss-plugin && $(PNPM) run typecheck

pkg-postcss-clean: ## Clean PostCSS plugin package
	@echo "$(YELLOW)Cleaning postcss-plugin package...$(NC)"
	cd packages/postcss-plugin && $(PNPM) run clean

# Jest package
pkg-jest-build: ## Build Jest package
	@echo "$(YELLOW)Building jest package...$(NC)"
	cd packages/jest && $(PNPM) run build

pkg-jest-lint: ## Lint Jest package
	@echo "$(YELLOW)Linting jest package...$(NC)"
	cd packages/jest && $(PNPM) run lint

pkg-jest-test: ## Test Jest package
	@echo "$(YELLOW)Testing jest package...$(NC)"
	cd packages/jest && $(PNPM) run test

pkg-jest-typecheck: ## Typecheck Jest package
	@echo "$(YELLOW)Typechecking jest package...$(NC)"
	cd packages/jest && $(PNPM) run typecheck

pkg-jest-clean: ## Clean Jest package
	@echo "$(YELLOW)Cleaning jest package...$(NC)"
	cd packages/jest && $(PNPM) run clean

# Design system package
pkg-design-build: ## Build design system package
	@echo "$(YELLOW)Building design-system package...$(NC)"
	cd packages/design-system && $(PNPM) run build

pkg-design-lint: ## Lint design system package
	@echo "$(YELLOW)Linting design-system package...$(NC)"
	cd packages/design-system && $(PNPM) run lint

pkg-design-test: ## Test design system package
	@echo "$(YELLOW)Testing design-system package...$(NC)"
	cd packages/design-system && $(PNPM) run test

pkg-design-typecheck: ## Typecheck design system package
	@echo "$(YELLOW)Typechecking design-system package...$(NC)"
	cd packages/design-system && $(PNPM) run typecheck

pkg-design-clean: ## Clean design system package
	@echo "$(YELLOW)Cleaning design-system package...$(NC)"
	cd packages/design-system && $(PNPM) run clean

# Playwright package
pkg-playwright-build: ## Build Playwright package
	@echo "$(YELLOW)Building playwright package...$(NC)"
	cd packages/playwright && $(PNPM) run build

pkg-playwright-lint: ## Lint Playwright package
	@echo "$(YELLOW)Linting playwright package...$(NC)"
	cd packages/playwright && $(PNPM) run lint

pkg-playwright-test: ## Test Playwright package
	@echo "$(YELLOW)Testing playwright package...$(NC)"
	cd packages/playwright && $(PNPM) run test

pkg-playwright-typecheck: ## Typecheck Playwright package
	@echo "$(YELLOW)Typechecking playwright package...$(NC)"
	cd packages/playwright && $(PNPM) run typecheck

pkg-playwright-clean: ## Clean Playwright package
	@echo "$(YELLOW)Cleaning playwright package...$(NC)"
	cd packages/playwright && $(PNPM) run clean

# ESLint config package
pkg-eslint-build: ## Build ESLint config package
	@echo "$(YELLOW)Building eslint-config package...$(NC)"
	cd packages/eslint-config && $(PNPM) run build

pkg-eslint-lint: ## Lint ESLint config package
	@echo "$(YELLOW)Linting eslint-config package...$(NC)"
	cd packages/eslint-config && $(PNPM) run lint

pkg-eslint-test: ## Test ESLint config package
	@echo "$(YELLOW)Testing eslint-config package...$(NC)"
	cd packages/eslint-config && $(PNPM) run test

pkg-eslint-typecheck: ## Typecheck ESLint config package
	@echo "$(YELLOW)Typechecking eslint-config package...$(NC)"
	cd packages/eslint-config && $(PNPM) run typecheck

pkg-eslint-clean: ## Clean ESLint config package
	@echo "$(YELLOW)Cleaning eslint-config package...$(NC)"
	cd packages/eslint-config && $(PNPM) run clean

# TypeScript config package
pkg-typescript-build: ## Build TypeScript config package
	@echo "$(YELLOW)Building typescript-config package...$(NC)"
	cd packages/typescript-config && $(PNPM) run build

pkg-typescript-lint: ## Lint TypeScript config package
	@echo "$(YELLOW)Linting typescript-config package...$(NC)"
	cd packages/typescript-config && $(PNPM) run lint

pkg-typescript-test: ## Test TypeScript config package
	@echo "$(YELLOW)Testing typescript-config package...$(NC)"
	cd packages/typescript-config && $(PNPM) run test

pkg-typescript-typecheck: ## Typecheck TypeScript config package
	@echo "$(YELLOW)Typechecking typescript-config package...$(NC)"
	cd packages/typescript-config && $(PNPM) run typecheck

pkg-typescript-clean: ## Clean TypeScript config package
	@echo "$(YELLOW)Cleaning typescript-config package...$(NC)"
	cd packages/typescript-config && $(PNPM) run clean

# Individual Rust crate commands
# Compiler crate
crate-compiler-build: ## Build Rust compiler crate
	@echo "$(YELLOW)Building stylex-rs-compiler crate...$(NC)"
	cd crates/stylex-rs-compiler && $(PNPM) run build

crate-compiler-format: ## Format Rust compiler crate
	@echo "$(YELLOW)Formatting stylex-rs-compiler crate...$(NC)"
	cd crates/stylex-rs-compiler && $(PNPM) run format

crate-compiler-lint: ## Lint Rust compiler crate
	@echo "$(YELLOW)Linting stylex-rs-compiler crate...$(NC)"
	cd crates/stylex-rs-compiler && $(PNPM) run lint:check

crate-compiler-clean: ## Clean Rust compiler crate
	@echo "$(YELLOW)Cleaning stylex-rs-compiler crate...$(NC)"
	cd crates/stylex-rs-compiler && $(PNPM) run clean

crate-compiler-docs: ## Generate docs for Rust compiler crate
	@echo "$(YELLOW)Generating docs for stylex-rs-compiler crate...$(NC)"
	cd crates/stylex-rs-compiler && $(CARGO) doc --no-deps

# Shared crate
crate-shared-build: ## Build shared crate
	@echo "$(YELLOW)Building stylex-shared crate...$(NC)"
	cd crates/stylex-shared && $(CARGO) build --release

crate-shared-format: ## Format shared crate
	@echo "$(YELLOW)Formatting stylex-shared crate...$(NC)"
	cd crates/stylex-shared && $(PNPM) run format

crate-shared-lint: ## Lint shared crate
	@echo "$(YELLOW)Linting stylex-shared crate...$(NC)"
	cd crates/stylex-shared && $(PNPM) run lint:check

crate-shared-clean: ## Clean shared crate
	@echo "$(YELLOW)Cleaning stylex-shared crate...$(NC)"
	cd crates/stylex-shared && $(PNPM) run clean

crate-shared-docs: ## Generate docs for shared crate
	@echo "$(YELLOW)Generating docs for stylex-shared crate...$(NC)"
	cd crates/stylex-shared && $(CARGO) doc --no-deps

# Path resolver crate
crate-resolver-build: ## Build path resolver crate
	@echo "$(YELLOW)Building stylex-path-resolver crate...$(NC)"
	cd crates/stylex-path-resolver && $(CARGO) build --release

crate-resolver-format: ## Format path resolver crate
	@echo "$(YELLOW)Formatting stylex-path-resolver crate...$(NC)"
	cd crates/stylex-path-resolver && $(CARGO) fmt

crate-resolver-lint: ## Lint path resolver crate
	@echo "$(YELLOW)Linting stylex-path-resolver crate...$(NC)"
	cd crates/stylex-path-resolver && $(CARGO) clippy --all-targets --all-features -- -D warnings

crate-resolver-clean: ## Clean path resolver crate
	@echo "$(YELLOW)Cleaning stylex-path-resolver crate...$(NC)"
	cd crates/stylex-path-resolver && $(CARGO) clean

crate-resolver-docs: ## Generate docs for path resolver crate
	@echo "$(YELLOW)Generating docs for stylex-path-resolver crate...$(NC)"
	cd crates/stylex-path-resolver && $(CARGO) doc --no-deps

# Test parser crate
crate-parser-build: ## Build test parser crate
	@echo "$(YELLOW)Building stylex-test-parser crate...$(NC)"
	cd crates/stylex-test-parser && $(PNPM) run build

crate-parser-format: ## Format test parser crate
	@echo "$(YELLOW)Formatting stylex-test-parser crate...$(NC)"
	cd crates/stylex-test-parser && $(PNPM) run format

crate-parser-lint: ## Lint test parser crate
	@echo "$(YELLOW)Linting stylex-test-parser crate...$(NC)"
	cd crates/stylex-test-parser && $(PNPM) run lint:check

crate-parser-clean: ## Clean test parser crate
	@echo "$(YELLOW)Cleaning stylex-test-parser crate...$(NC)"
	cd crates/stylex-test-parser && $(PNPM) run clean

crate-parser-docs: ## Generate docs for test parser crate
	@echo "$(YELLOW)Generating docs for stylex-test-parser crate...$(NC)"
	cd crates/stylex-test-parser && $(CARGO) doc --no-deps

# =============================================================================
# App Commands
# =============================================================================

apps-build: ## Build all example apps
	@echo "$(YELLOW)Building all example apps...$(NC)"
	$(TURBO) run build --filter="./apps/*"
	@echo "$(GREEN)All apps built successfully!$(NC)"

apps-dev: ## Start development servers for all apps
	@echo "$(YELLOW)Starting development servers for all apps...$(NC)"
	$(TURBO) run dev --filter="./apps/*" --parallel
	@echo "$(GREEN)All app dev servers started!$(NC)"

apps-clean: ## Clean all app build artifacts
	@echo "$(YELLOW)Cleaning all app build artifacts...$(NC)"
	$(TURBO) run clean --filter="./apps/*"
	@echo "$(GREEN)All app artifacts cleaned!$(NC)"

# Individual app commands
app-nextjs-dev: ## Start Next.js example app in development mode
	@echo "$(YELLOW)Starting Next.js example app...$(NC)"
	cd apps/nextjs-example && $(PNPM) run dev

app-nextjs-build: ## Build Next.js example app
	@echo "$(YELLOW)Building Next.js example app...$(NC)"
	cd apps/nextjs-example && $(PNPM) run build

app-nextjs-serve: ## Serve Next.js example app (requires build first)
	@echo "$(YELLOW)Serving Next.js example app...$(NC)"
	cd apps/nextjs-example && $(PNPM) run serve

app-vite-dev: ## Start Vite example app in development mode
	@echo "$(YELLOW)Starting Vite example app...$(NC)"
	cd apps/vite-unplugin-example && $(PNPM) run dev

app-vite-build: ## Build Vite example app
	@echo "$(YELLOW)Building Vite example app...$(NC)"
	cd apps/vite-unplugin-example && $(PNPM) run build

app-vite-serve: ## Serve Vite example app (requires build first)
	@echo "$(YELLOW)Serving Vite example app...$(NC)"
	cd apps/vite-unplugin-example && $(PNPM) run serve

app-webpack-dev: ## Start Webpack example app in development mode
	@echo "$(YELLOW)Starting Webpack example app...$(NC)"
	cd apps/webpack-example && $(PNPM) run start

app-webpack-build: ## Build Webpack example app
	@echo "$(YELLOW)Building Webpack example app...$(NC)"
	cd apps/webpack-example && $(PNPM) run build

app-rollup-dev: ## Start Rollup example app in development mode
	@echo "$(YELLOW)Starting Rollup example app...$(NC)"
	cd apps/rollup-example && $(PNPM) run dev

app-rollup-build: ## Build Rollup example app
	@echo "$(YELLOW)Building Rollup example app...$(NC)"
	cd apps/rollup-example && $(PNPM) run build

# Serve multiple apps simultaneously
apps-serve-common: ## Serve commonly used example apps (Next.js, Vite, Webpack)
	@echo "$(YELLOW)Starting multiple app servers...$(NC)"
	@echo "$(BLUE)Next.js app will be available at http://localhost:3000$(NC)"
	@echo "$(BLUE)Vite app will be available at http://localhost:5173$(NC)"
	@echo "$(BLUE)Press Ctrl+C to stop all servers$(NC)"
	@(cd apps/nextjs-example && $(PNPM) run dev) & \
	(cd apps/vite-unplugin-example && $(PNPM) run dev) & \
	wait

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
