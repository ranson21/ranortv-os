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
