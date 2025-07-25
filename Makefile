# Minimal Production Makefile for Buildroot Integration
# Separates build-time from runtime dependencies

.PHONY: help build-static build-container extract-binary buildroot-prepare clean-build dev-setup

# Configuration
PROJECT_NAME = media-launcher
RUST_TARGET = x86_64-unknown-linux-musl
CONTAINER_NAME = media-launcher-builder
BUILDROOT_DIR = ../buildroot
OVERLAY_DIR = $(BUILDROOT_DIR)/board/media-os/rootfs_overlay

help:
	@echo "Media OS Launcher - Production Build System"
	@echo ""
	@echo "Development:"
	@echo "  dev-setup          - Setup development environment"
	@echo "  build-dev          - Build for development/testing"
	@echo "  run-dev            - Run development version"
	@echo ""
	@echo "Production (Minimal Runtime):"
	@echo "  build-static       - Build statically linked binary"
	@echo "  build-container    - Build in isolated container"
	@echo "  extract-binary     - Extract binary from container"
	@echo ""
	@echo "Buildroot Integration:"
	@echo "  buildroot-prepare  - Prepare for Buildroot integration"
	@echo "  buildroot-build    - Build complete Buildroot image"
	@echo ""
	@echo "Utilities:"
	@echo "  size-analysis      - Analyze binary size"
	@echo "  runtime-deps       - Show runtime dependencies"
	@echo "  clean-build        - Clean all build artifacts"

# ===============================
# Development Environment
# ===============================

dev-setup:
	@echo "🔧 Setting up development environment..."
	@if ! command -v rustc >/dev/null 2>&1; then \
		curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y; \
		. $(HOME)/.cargo/env; \
	fi
	@. $(HOME)/.cargo/env && rustup target add $(RUST_TARGET)
	@. $(HOME)/.cargo/env && rustup component add rustfmt clippy
	@mkdir -p ui src
	@echo "✅ Development environment ready"

build-dev:
	@echo "🔨 Building development version..."
	@cd launcher && cargo build

run-dev:
	@echo "🚀 Running development version..."
	@cd launcher && cargo run

# ===============================
# Static Binary Production
# ===============================

build-static:
	@echo "🔨 Building static binary for $(RUST_TARGET)..."
	@cargo build --release --target $(RUST_TARGET)
	@echo "✅ Static binary built: target/$(RUST_TARGET)/release/$(PROJECT_NAME)"

# Build in completely isolated container
build-container:
	@echo "🐳 Building in isolated container..."
	@docker build -t $(CONTAINER_NAME) -f Dockerfile.builder .
	@echo "✅ Container build complete"

extract-binary:
	@echo "📦 Extracting binary from container..."
	@mkdir -p dist
	@docker create --name temp-container $(CONTAINER_NAME)
	@docker cp temp-container:/usr/bin/$(PROJECT_NAME) dist/$(PROJECT_NAME)
	@docker rm temp-container
	@strip dist/$(PROJECT_NAME)
	@echo "✅ Binary extracted to dist/$(PROJECT_NAME)"

# ===============================
# Buildroot Integration
# ===============================

buildroot-prepare: extract-binary
	@echo "📋 Preparing Buildroot integration..."
	@mkdir -p $(OVERLAY_DIR)/usr/bin
	@mkdir -p $(OVERLAY_DIR)/etc/systemd/system
	@mkdir -p $(OVERLAY_DIR)/apps
	@cp dist/$(PROJECT_NAME) $(OVERLAY_DIR)/usr/bin/
	@chmod +x $(OVERLAY_DIR)/usr/bin/$(PROJECT_NAME)
	@cp buildroot/media-launcher.service $(OVERLAY_DIR)/etc/systemd/system/
	@cp -r buildroot/rootfs_overlay/* $(OVERLAY_DIR)/ 2>/dev/null || true
	@echo "✅ Buildroot overlay prepared"

buildroot-build: buildroot-prepare
	@echo "🏗️  Building complete Buildroot image..."
	@cd $(BUILDROOT_DIR) && make media_os_defconfig
	@cd $(BUILDROOT_DIR) && make
	@echo "✅ Buildroot image built: $(BUILDROOT_DIR)/output/images/"

# ===============================
# Analysis & Debugging
# ===============================

size-analysis:
	@echo "📊 Binary size analysis..."
	@if [ -f "dist/$(PROJECT_NAME)" ]; then \
		ls -lh dist/$(PROJECT_NAME); \
		file dist/$(PROJECT_NAME); \
		echo ""; \
		echo "📦 Size breakdown:"; \
		size dist/$(PROJECT_NAME); \
	else \
		echo "❌ Binary not found. Run 'make extract-binary' first"; \
	fi

runtime-deps:
	@echo "🔍 Runtime dependencies analysis..."
	@if [ -f "dist/$(PROJECT_NAME)" ]; then \
		echo "Dynamic libraries:"; \
		ldd dist/$(PROJECT_NAME) 2>/dev/null || echo "✅ Statically linked (no dynamic deps)"; \
		echo ""; \
		echo "System calls used:"; \
		objdump -T dist/$(PROJECT_NAME) 2>/dev/null | head -20 || echo "No symbols (stripped)"; \
	else \
		echo "❌ Binary not found. Run 'make extract-binary' first"; \
	fi

# ===============================
# Cleanup
# ===============================

clean-build:
	@echo "🧹 Cleaning build artifacts..."
	@cargo clean
	@rm -rf dist/
	@docker rmi $(CONTAINER_NAME) 2>/dev/null || true
	@echo "✅ Build artifacts cleaned"

# ===============================
# Minimal Binary Configuration
# ===============================

configure-minimal:
	@echo "⚙️  Configuring for minimal binary..."
	@echo '[profile.release]' > Cargo.toml.minimal
	@echo 'opt-level = "z"' >> Cargo.toml.minimal
	@echo 'lto = true' >> Cargo.toml.minimal
	@echo 'codegen-units = 1' >> Cargo.toml.minimal
	@echo 'panic = "abort"' >> Cargo.toml.minimal
	@echo 'strip = true' >> Cargo.toml.minimal
	@echo ''
	@echo '[dependencies]' >> Cargo.toml.minimal
	@cat Cargo.toml | grep -A 100 '\[dependencies\]' | tail -n +2 >> Cargo.toml.minimal
	@mv Cargo.toml Cargo.toml.dev
	@mv Cargo.toml.minimal Cargo.toml
	@echo "✅ Configured for minimal binary (Cargo.toml.dev backed up)"

restore-dev:
	@echo "🔄 Restoring development configuration..."
	@mv Cargo.toml.dev Cargo.toml
	@echo "✅ Development configuration restored"