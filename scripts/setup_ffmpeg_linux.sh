#!/bin/bash

# 检查是否为 Linux
if [[ "$(uname)" != "Linux" ]]; then
    echo "此脚本仅适用于 Linux"
    exit 1
fi

# 检查是否有 root 权限
if [ "$EUID" -ne 0 ]; then 
    echo "请使用 sudo 运行此脚本"
    exit 1
fi

# 检测包管理器并安装 FFmpeg
if command -v apt-get &> /dev/null; then
    # Debian/Ubuntu
    echo "检测到 apt 包管理器"
    apt-get update
    apt-get install -y ffmpeg
elif command -v dnf &> /dev/null; then
    # Fedora
    echo "检测到 dnf 包管理器"
    dnf install -y https://download1.rpmfusion.org/free/fedora/rpmfusion-free-release-$(rpm -E %fedora).noarch.rpm
    dnf install -y ffmpeg
elif command -v yum &> /dev/null; then
    # CentOS/RHEL
    echo "检测到 yum 包管理器"
    yum install -y epel-release
    yum install -y ffmpeg
elif command -v pacman &> /dev/null; then
    # Arch Linux
    echo "检测到 pacman 包管理器"
    pacman -Sy ffmpeg --noconfirm
else
    echo "未检测到支持的包管理器"
    exit 1
fi

# 验证安装
if command -v ffmpeg &> /dev/null; then
    echo "FFmpeg 安装成功！"
    echo "版本信息："
    ffmpeg -version
else
    echo "FFmpeg 安装失败！"
    exit 1
fi
