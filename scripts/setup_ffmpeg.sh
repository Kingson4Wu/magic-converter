#!/bin/bash

# 检查是否为 macOS
if [[ "$(uname)" != "Darwin" ]]; then
    echo "此脚本仅适用于 macOS"
    exit 1
fi

# 检查是否已安装 Homebrew
if ! command -v brew &> /dev/null; then
    echo "正在安装 Homebrew..."
    /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
fi

# 检查是否已安装 FFmpeg
if ! command -v ffmpeg &> /dev/null; then
    echo "正在安装 FFmpeg..."
    brew install ffmpeg
else
    echo "FFmpeg 已安装，正在检查更新..."
    brew upgrade ffmpeg
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
