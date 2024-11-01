#!/bin/bash

# 安装交叉编译工具链
sudo apt update && sudo apt install -f
sudo apt install -y \
    gcc-aarch64-linux-gnu \
    g++-aarch64-linux-gnu \
    crossbuild-essential-arm64

# 安装 pkg-config
sudo apt install -y pkg-config

# 安装 ARM64 版本的依赖库
sudo apt install -y --no-install-recommends \
    gettext:arm64 \
    libssl-dev:arm64 \
    libmpv-dev:arm64 \
    libgtk-4-dev:arm64 \
    libadwaita-1-dev:arm64 \
    libgstreamer1.0-dev:arm64 \
    libgstreamer-plugins-base1.0-dev:arm64 \
    libgstreamer-plugins-good1.0-dev:arm64 \
    libgstreamer-plugins-bad1.0-dev:arm64 \
    gstreamer1.0-libav:arm64
