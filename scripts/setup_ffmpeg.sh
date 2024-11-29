#!/bin/bash

# 设置错误处理
set -e

# 检查是否为 macOS
if [[ "$(uname)" != "Darwin" ]]; then
    echo "此脚本仅适用于 macOS"
    exit 1
fi

# 检查网络连接
echo "检查网络连接..."
if ! ping -c 1 google.com &> /dev/null; then
    echo "警告: 网络连接可能不稳定，这可能会影响安装过程"
fi

# 检查是否有足够的磁盘空间（至少需要500MB）
echo "检查磁盘空间..."
available_space=$(df -k / | awk 'NR==2 {print $4}')
if [ "$available_space" -lt 512000 ]; then
    echo "错误: 磁盘空间不足，至少需要500MB可用空间"
    exit 1
fi

# 检查是否已安装 Homebrew
echo "检查 Homebrew..."
if ! command -v brew &> /dev/null; then
    echo "正在安装 Homebrew..."
    /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
    
    # 检查 Homebrew 是否安装成功
    if ! command -v brew &> /dev/null; then
        echo "Homebrew 安装失败！"
        exit 1
    fi
fi

# 检查并更新 Homebrew
echo "更新 Homebrew..."
brew update

# 检查是否已安装 FFmpeg
echo "检查 FFmpeg..."
if ! command -v ffmpeg &> /dev/null; then
    echo "正在安装 FFmpeg..."
    if ! brew install ffmpeg; then
        echo "FFmpeg 安装失败！"
        exit 1
    fi
else
    echo "FFmpeg 已安装，检查更新..."
    brew upgrade ffmpeg || echo "FFmpeg 已是最新版本"
fi

# 验证 FFmpeg 安装和功能
echo "验证 FFmpeg 安装..."
if command -v ffmpeg &> /dev/null; then
    echo "FFmpeg 安装成功！"
    
    # 检查 FFmpeg 版本和编解码器支持
    echo "版本信息："
    ffmpeg -version
    
    echo "检查编解码器支持..."
    if ! ffmpeg -codecs | grep -q "h264"; then
        echo "警告: 未检测到 H.264 编解码器支持"
    else
        echo "H.264 编解码器支持正常"
    fi
    
    # 检查环境变量
    if [[ ":$PATH:" != *"$(brew --prefix)/bin:"* ]]; then
        echo "警告: FFmpeg 可能不在系统 PATH 中"
        echo "建议将以下行添加到您的 ~/.zshrc 或 ~/.bash_profile 中："
        echo 'export PATH="$(brew --prefix)/bin:$PATH"'
    fi
else
    echo "FFmpeg 安装失败！"
    exit 1
fi

echo "设置完成！"
