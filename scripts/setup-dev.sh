#!/bin/bash
# RanorTV Development Environment Setup Script

set -e

echo "🚀 Setting up RanorTV development environment..."

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Helper functions
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if running on supported OS
check_os() {
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        print_status "Detected Linux OS"
        if command -v apt-get >/dev/null 2>&1; then
            PKG_MANAGER="apt"
        elif command -v yum >/dev/null 2>&1; then
            PKG_MANAGER="yum"
        elif command -v pacman >/dev/null 2>&1; then
            PKG_MANAGER="pacman"
        else
            print_error "Unsupported package manager"
            exit 1
        fi
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        print_status "Detected macOS"
        PKG_MANAGER="brew"
    else
        print_error "Unsupported operating system: $OSTYPE"
        exit 1
    fi
}

# Install system dependencies
install_system_deps() {
    print_status "Installing system dependencies..."
    
    case $PKG_MANAGER in
        "apt")
            sudo apt-get update
            sudo apt-get install -y \
                build-essential \
                pkg-config \
                libgl1-mesa-dev \
                libxrandr-dev \
                libxinerama-dev \
                libxcursor-dev \
                libxi-dev \
                libxkbcommon-dev \
                libwayland-dev \
                libxkbcommon-x11-dev \
                curl \
                git \
                file \
                wget \
                cpio \
                unzip \
                rsync \
                bc \
                util-linux \
                nodejs \
                npm
            ;;
        "yum")
            sudo yum groupinstall -y "Development Tools"
            sudo yum install -y \
                mesa-libGL-devel \
                libXrandr-devel \
                libXinerama-devel \
                libXcursor-devel \
                libXi-devel \
                libxkbcommon-devel \
                wayland-devel \
                curl \
                git \
                file \
                wget \
                cpio \
                unzip \
                rsync \
                bc \
                util-linux \
                nodejs \
                npm
            ;;
        "pacman")
            sudo pacman -S --needed \
                base-devel \
                mesa \
                libxrandr \
                libxinerama \
                libxcursor \
                libxi \
                libxkbcommon \
                libxkbcommon-x11 \
                wayland \
                curl \
                git \
                file \
                wget \
                cpio \
                unzip \
                rsync \
                bc \
                util-linux \
                nodejs \
                npm
            ;;
        "brew")
            brew install \
                curl \
                git \
                file \
                wget \
                cpio \
                rsync \
                node \
                npm
            ;;
    esac
    
    print_success "System dependencies installed"
}

# Install Rust
install_rust() {
    print_status "Installing Rust..."
    
    if command -v rustc >/dev/null 2>&1; then
        print_warning "Rust already installed ($(rustc --version))"
        print_status "Updating Rust..."
        rustup update
    else
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source ~/.cargo/env
        print_success "Rust installed successfully"
    fi
    
    # Ensure cargo is in PATH
    export PATH="$HOME/.cargo/bin:$PATH"
    
    # Add required targets
    print_status "Adding Rust targets..."
    rustup target add x86_64-unknown-linux-musl
    rustup component add rustfmt clippy
    
    print_success "Rust setup complete"
}

# Setup project structure
setup_project_structure() {
    print_status "Setting up project structure..."
    
    # Create main directories
    mkdir -p launcher/{src,ui/{components,styles},assets/{icons,fonts,images}}
    mkdir -p buildroot/{configs,board/ranortv/rootfs_overlay/{usr/bin,etc/{init.d,ranortv,systemd/system},apps,home/ranortv}}
    mkdir -p buildroot/package/ranortv-launcher
    mkdir -p scripts docs/{images} tests/{unit,integration,fixtures/sample-apps}
    mkdir -p tools/{app-store-server,app-packager/src,device-simulator}
    mkdir -p dist examples/{sample-apps,configs} ci/{github-actions,docker}
    
    # Create basic Cargo.toml
    cat > launcher/Cargo.toml << 'EOF'
[package]
name = "ranortv-launcher"
version = "0.1.0"
edition = "2021"

[dependencies]
slint = { version = "1.3", default-features = false, features = [
    "std",
    "renderer-gl",
    "accessibility",
] }
serde = { version = "1.0", default-features = false, features = ["derive"] }
serde_json = { version = "1.0", default-features = false }

[build-dependencies]
slint-build = "1.3"

[profile.release]
opt-level = "z"
lto = true
codegen-units = 1
panic = "abort"
strip = true

[target.x86_64-unknown-linux-musl]
rustflags = ["-C", "target-feature=+crt-static"]
EOF

    # Create build.rs
    cat > launcher/build.rs << 'EOF'
fn main() {
    slint_build::compile("ui/app_window.slint").unwrap();
}
EOF

    # Create basic main.rs placeholder
    cat > launcher/src/main.rs << 'EOF'
// RanorTV Launcher - Apple TV-style Media OS
// This is a placeholder - replace with the complete launcher code

fn main() {
    println!("🎬 RanorTV Launcher");
    println!("Replace this file with the complete launcher code from the artifacts");
    println!("See README.md for setup instructions");
}
EOF

    # Create basic UI file placeholder
    mkdir -p launcher/ui
    cat > launcher/ui/app_window.slint << 'EOF'
// RanorTV UI Definition - Apple TV-style Interface
// This is a placeholder - replace with the complete UI code

import { Window } from "std-widgets.slint";

export component AppWindow inherits Window {
    title: "RanorTV - Replace with complete UI";
    
    Text {
        text: "🎬 RanorTV\nReplace this UI file with the complete Slint code";
        font-size: 24px;
        color: white;
        horizontal-alignment: center;
        vertical-alignment: center;
    }
}
EOF

    print_success "Project structure created"
}

# Setup development tools
setup_dev_tools() {
    print_status "Setting up development tools..."
    
    # Install cargo tools
    export PATH="$HOME/.cargo/bin:$PATH"
    cargo install cargo-watch cargo-edit || print_warning "Some cargo tools failed to install"
    
    # Setup git hooks (if in git repo)
    if [ -d ".git" ]; then
        print_status "Setting up git hooks..."
        mkdir -p .git/hooks
        cat > .git/hooks/pre-commit << 'EOF'
#!/bin/bash
echo "Running pre-commit checks..."
cd launcher
cargo fmt --check
cargo clippy -- -D warnings
EOF
        chmod +x .git/hooks/pre-commit
    fi
    
    print_success "Development tools setup complete"
}

# Create sample apps
create_sample_apps() {
    print_status "Creating sample apps..."
    
    # YouTube app
    mkdir -p buildroot/board/ranortv/rootfs_overlay/apps/youtube
    cat > buildroot/board/ranortv/rootfs_overlay/apps/youtube/app.json << 'EOF'
{
    "id": "youtube",
    "name": "YouTube",
    "description": "Watch videos online",
    "version": "1.0.0",
    "category": "Entertainment",
    "executable_path": "./run.sh",
    "icon_path": "./icon.png"
}
EOF

    cat > buildroot/board/ranortv/rootfs_overlay/apps/youtube/run.sh << 'EOF'
#!/bin/bash
echo "🎥 YouTube app launched!"
echo "Opening https://youtube.com..."
sleep 3
echo "YouTube session ended"
EOF
    chmod +x buildroot/board/ranortv/rootfs_overlay/apps/youtube/run.sh

    # Plex app
    mkdir -p buildroot/board/ranortv/rootfs_overlay/apps/plex
    cat > buildroot/board/ranortv/rootfs_overlay/apps/plex/app.json << 'EOF'
{
    "id": "plex",
    "name": "Plex",
    "description": "Media streaming platform",
    "version": "2.1.0",
    "category": "Entertainment",
    "executable_path": "./run.sh",
    "icon_path": "./icon.png"
}
EOF

    cat > buildroot/board/ranortv/rootfs_overlay/apps/plex/run.sh << 'EOF'
#!/bin/bash
echo "📺 Plex app launched!"
echo "Starting media server..."
sleep 3
echo "Plex session ended"
EOF
    chmod +x buildroot/board/ranortv/rootfs_overlay/apps/plex/run.sh

    # Netflix app
    mkdir -p buildroot/board/ranortv/rootfs_overlay/apps/netflix
    cat > buildroot/board/ranortv/rootfs_overlay/apps/netflix/app.json << 'EOF'
{
    "id": "netflix",
    "name": "Netflix",
    "description": "Stream movies and TV shows",
    "version": "3.0.1",
    "category": "Entertainment",
    "executable_path": "./run.sh",
    "icon_path": "./icon.png"
}
EOF

    cat > buildroot/board/ranortv/rootfs_overlay/apps/netflix/run.sh << 'EOF'
#!/bin/bash
echo "🍿 Netflix app launched!"
echo "Loading your watchlist..."
sleep 3
echo "Netflix session ended"
EOF
    chmod +x buildroot/board/ranortv/rootfs_overlay/apps/netflix/run.sh

    print_success "Sample apps created"
}

# Create configuration files
create_config_files() {
    print_status "Creating configuration files..."
    
    # System configuration
    cat > buildroot/board/ranortv/rootfs_overlay/etc/ranortv/config.json << 'EOF'
{
    "ui": {
        "theme": "dark",
        "animations": true,
        "grid_columns": 5,
        "tile_size": 200,
        "focus_scale": 1.05
    },
    "system": {
        "auto_updates": true,
        "parental_controls": false,
        "network_timeout": 30,
        "boot_splash": true
    },
    "apps": {
        "sandbox_level": "strict",
        "allow_network": false,
        "data_directory": "/apps",
        "auto_install_updates": true
    },
    "store": {
        "store_url": "https://store.ranortv.com",
        "auto_check_updates": true,
        "featured_refresh_interval": 3600
    }
}
EOF

    # Init script
    cat > buildroot/board/ranortv/rootfs_overlay/etc/init.d/S99ranortv << 'EOF'
#!/bin/sh
#
# Start RanorTV launcher
#

case "$1" in
  start)
    echo "Starting RanorTV launcher..."
    # Set up display
    export DISPLAY=:0
    # Start launcher
    /usr/bin/ranortv-launcher &
    ;;
  stop)
    echo "Stopping RanorTV launcher..."
    killall ranortv-launcher 2>/dev/null || true
    ;;
  restart)
    $0 stop
    $0 start
    ;;
  *)
    echo "Usage: $0 {start|stop|restart}"
    exit 1
esac

exit $?
EOF
    chmod +x buildroot/board/ranortv/rootfs_overlay/etc/init.d/S99ranortv

    # Systemd service
    cat > buildroot/board/ranortv/rootfs_overlay/etc/systemd/system/ranortv.service << 'EOF'
[Unit]
Description=RanorTV Media Launcher
After=graphical.target

[Service]
Type=simple
User=ranortv
Environment=DISPLAY=:0
ExecStart=/usr/bin/ranortv-launcher
Restart=always
RestartSec=3

[Install]
WantedBy=graphical.target
EOF

    print_success "Configuration files created"
}

# Setup mock app store server
setup_app_store() {
    print_status "Setting up mock app store server..."
    
    cat > tools/app-store-server/package.json << 'EOF'
{
    "name": "ranortv-app-store",
    "version": "1.0.0",
    "description": "Mock app store server for RanorTV development",
    "main": "server.js",
    "scripts": {
        "start": "node server.js",
        "dev": "nodemon server.js"
    },
    "dependencies": {
        "express": "^4.18.0",
        "cors": "^2.8.5"
    },
    "devDependencies": {
        "nodemon": "^3.0.0"
    }
}
EOF

    cat > tools/app-store-server/server.js << 'EOF'
const express = require('express');
const cors = require('cors');

const app = express();
const PORT = process.env.PORT || 3000;

app.use(cors());
app.use(express.json());

// Mock app data
const apps = [
    {
        "id": "spotify",
        "name": "Spotify",
        "description": "Music streaming service",
        "version": "1.0.0",
        "category": "Music",
        "download_url": "https://store.ranortv.com/apps/spotify.tar.gz",
        "icon_url": "https://store.ranortv.com/icons/spotify.png",
        "rating": 4.8,
        "downloads": 1250000,
        "installed": false
    },
    {
        "id": "twitch",
        "name": "Twitch",
        "description": "Live streaming platform",
        "version": "2.1.0",
        "category": "Entertainment",
        "download_url": "https://store.ranortv.com/apps/twitch.tar.gz",
        "icon_url": "https://store.ranortv.com/icons/twitch.png",
        "rating": 4.6,
        "downloads": 890000,
        "installed": false
    }
];

// API endpoints
app.get('/api/apps', (req, res) => {
    res.json({ apps });
});

app.get('/api/apps/:id', (req, res) => {
    const app = apps.find(a => a.id === req.params.id);
    if (app) {
        res.json(app);
    } else {
        res.status(404).json({ error: 'App not found' });
    }
});

app.get('/api/featured', (req, res) => {
    res.json({ apps: apps.slice(0, 3) });
});

app.listen(PORT, () => {
    console.log(`🏪 RanorTV App Store server running on port ${PORT}`);
});
EOF

    # Install npm dependencies if node is available
    if command -v npm >/dev/null 2>&1; then
        print_status "Installing app store dependencies..."
        cd tools/app-store-server
        npm install 2>/dev/null || print_warning "Failed to install npm dependencies"
        cd - > /dev/null
    fi

    print_success "Mock app store server created"
}

# Create Docker files
create_docker_files() {
    print_status "Creating Docker configuration..."
    
    mkdir -p ci/docker
    
    # Development Dockerfile
    cat > ci/docker/dev.Dockerfile << 'EOF'
FROM rust:1.75-slim

RUN apt-get update && apt-get install -y \
    pkg-config \
    libgl1-mesa-dev \
    libxrandr-dev \
    libxinerama-dev \
    libxcursor-dev \
    libxi-dev \
    libxkbcommon-dev \
    libwayland-dev \
    libxkbcommon-x11-dev \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

RUN rustup target add x86_64-unknown-linux-musl
RUN rustup component add rustfmt clippy

WORKDIR /workspace
EOF

    # Buildroot Dockerfile
    cat > ci/docker/buildroot.Dockerfile << 'EOF'
FROM ubuntu:22.04

RUN apt-get update && apt-get install -y \
    build-essential \
    file \
    wget \
    cpio \
    unzip \
    rsync \
    bc \
    git \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /workspace
EOF

    print_success "Docker configuration created"
}

# Check prerequisites
check_prerequisites() {
    print_status "Checking prerequisites..."
    
    # Check for required commands
    local missing_deps=()
    
    for cmd in curl git; do
        if ! command -v $cmd >/dev/null 2>&1; then
            missing_deps+=($cmd)
        fi
    done
    
    if [ ${#missing_deps[@]} -ne 0 ]; then
        print_error "Missing required dependencies: ${missing_deps[*]}"
        print_status "Please install them and run this script again"
        exit 1
    fi
    
    print_success "Prerequisites check passed"
}

# Create final instructions
create_instructions() {
    cat > SETUP_COMPLETE.md << 'EOF'
# 🎉 RanorTV Setup Complete!

Your development environment is ready! Here's what was created:

## 📁 Project Structure
- `launcher/` - Main Rust application (placeholder code added)
- `buildroot/` - Complete OS build configuration
- `tools/` - Development utilities including mock app store
- `scripts/` - Build and utility scripts
- `docs/` - Documentation directory
- `tests/` - Test framework setup

## 🚀 Next Steps

### 1. Add the Complete Source Code
Replace the placeholder files with the complete code:
- Copy the Rust launcher code to `launcher/src/main.rs`
- Copy the Slint UI code to `launcher/ui/app_window.slint`
- Update `launcher/Cargo.toml` if needed

### 2. Build and Test
```bash
# Build the launcher
cd launcher
cargo build

# Run in development mode
cargo run

# Or use the Makefile
make build-dev
make run-dev
```

### 3. Start the App Store (Optional)
```bash
cd tools/app-store-server
npm start
```

### 4. Build Complete OS Image
```bash
make buildroot-build
```

## 🛠️ Available Commands
- `make dev-setup` - Setup development environment
- `make build-dev` - Build debug version
- `make run-dev` - Run launcher
- `make build-static` - Build production binary
- `make buildroot-build` - Build complete OS

## 📚 Documentation
- See `README.md` for complete documentation
- Check `docs/` directory for guides
- Sample apps are in `buildroot/board/ranortv/rootfs_overlay/apps/`

Happy coding! 🎬
EOF
    
    print_success "Setup instructions created (see SETUP_COMPLETE.md)"
}

# Main setup function
main() {
    echo "🎬 RanorTV Development Environment Setup"
    echo "========================================"
    echo
    
    check_prerequisites
    check_os
    install_system_deps
    install_rust
    setup_project_structure
    setup_dev_tools
    create_sample_apps
    create_config_files
    setup_app_store
    create_docker_files
    create_instructions
    
    echo
    print_success "🎉 RanorTV development environment setup complete!"
    echo
    echo "📋 Summary:"
    echo "  ✅ System dependencies installed"
    echo "  ✅ Rust toolchain configured"
    echo "  ✅ Project structure created"
    echo "  ✅ Sample apps and configs added"
    echo "  ✅ Development tools setup"
    echo "  ✅ Mock app store created"
    echo "  ✅ Docker configuration added"
    echo
    echo "📖 Next steps (see SETUP_COMPLETE.md):"
    echo "  1. Replace placeholder code with complete launcher implementation"
    echo "  2. Run 'make build-dev' to build the launcher"
    echo "  3. Run 'make run-dev' to test the launcher"
    echo "  4. Check README.md for complete documentation"
    echo
    echo "🚀 Happy coding!"
}

# Run main function
main "$@"