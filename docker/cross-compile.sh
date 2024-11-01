#!/bin/bash
# cross-compile.sh

export PKG_CONFIG_ALLOW_CROSS=1
export PKG_CONFIG_PATH=/usr/lib/aarch64-linux-gnu/pkgconfig
export PKG_CONFIG_SYSROOT_DIR=/usr/aarch64-linux-gnu
export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc

# GTK 和 GStreamer 相关环境变量
export GTK_TARGET_PATH=/usr/lib/aarch64-linux-gnu
export GST_PLUGIN_PATH=/usr/lib/aarch64-linux-gnu/gstreamer-1.0

# 执行实际的构建命令
cargo build --target aarch64-unknown-linux-gnu --release $@