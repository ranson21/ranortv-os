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
