#!/bin/bash

# 检查是否以 root 权限运行
if [ "$EUID" -ne 0 ]; then 
    echo "Please run as root"
    exit 1
fi

# 备份原有的 sources.list
cp /etc/apt/sources.list /etc/apt/sources.list.backup.$(date +%F)

# 添加 arm64 架构支持
dpkg --add-architecture arm64

# 确保 sources.list.d 目录存在
mkdir -p /etc/apt/sources.list.d

# 创建 arm64 专用源文件
cat > /etc/apt/sources.list.d/arm64.list << 'EOL'
deb [arch=arm64] http://ports.ubuntu.com/ubuntu-ports noble main restricted
deb [arch=arm64] http://ports.ubuntu.com/ubuntu-ports noble-updates main restricted
deb [arch=arm64] http://ports.ubuntu.com/ubuntu-ports noble universe
deb [arch=arm64] http://ports.ubuntu.com/ubuntu-ports noble-updates universe
deb [arch=arm64] http://ports.ubuntu.com/ubuntu-ports noble multiverse
deb [arch=arm64] http://ports.ubuntu.com/ubuntu-ports noble-updates multiverse
deb [arch=arm64] http://ports.ubuntu.com/ubuntu-ports noble-backports main restricted universe multiverse
deb [arch=arm64] http://ports.ubuntu.com/ubuntu-ports noble-security main restricted
deb [arch=arm64] http://ports.ubuntu.com/ubuntu-ports noble-security universe
deb [arch=arm64] http://ports.ubuntu.com/ubuntu-ports noble-security multiverse
EOL

# 修改现有的 sources.list，添加 [arch=amd64] 标识
sed -i '/^deb /s/^deb /deb [arch=amd64] /' /etc/apt/sources.list

# 更新包列表
apt update

echo "ARM64 sources setup completed. Original sources.list has been backed up to sources.list.backup.$(date +%F)"