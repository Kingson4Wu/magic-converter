编译命令（针对不同平台）：

Linux 静态编译（推荐使用 musl）：

bashCopy# 安装 musl 工具链
rustup target add x86_64-unknown-linux-musl

# 编译
cargo build --target x86_64-unknown-linux-musl --release

macOS 静态编译：

bashCopy# 编译
RUSTFLAGS="-C target-feature=+crt-static" cargo build --release

Windows 静态编译：

bashCopy# 安装 x86_64-pc-windows-msvc 工具链
rustup target add x86_64-pc-windows-msvc

# 编译
cargo build --target x86_64-pc-windows-msvc --release
注意事项：

需要安装 Rust 工具链和目标平台的编译工具
静态链接可能会增加二进制文件大小
不同平台可能需要略微调整编译配置

使用示例：
bashCopy# 运行程序
./mts-converter input.mts output.mp4
这个解决方案的优点：

完全静态链接
不依赖系统 FFmpeg 库
跨平台兼容
简单易用
直接从命令行传入输入输出文件

建议：

测试不同系统的兼容性
处理大文件时可能需要优化内存使用
添加更多错误处理和日志功能


====


不使用 FFmpeg 将 MTS 视频转换为 MP4 是比较困难的，因为这涉及到视频解码和编码的复杂过程。不过，我们可以探讨几种替代方案：

使用纯 Rust 的视频处理库

Alternative MTS to MP4 ConverterClick to open code

可选的开源库：

rustCopy// 可能的库（需要额外研究和安装）
// - gstreamer-rs：多媒体处理框架
// - mp4parse-rust：MP4文件解析
// - video-rs：实验性视频处理库
现实情况是，视频转换是一个非常复杂的过程，主要原因包括：

视频编解码需要处理：

视频流解码
音频流解码
数据流重新编码
多媒体容器格式转换


MTS 文件特点：

通常是 MPEG-2 传输流
需要专业的编解码能力
包含复杂的视频和音频编码



如果您一定要在 Rust 中处理，我建议的方案是：

使用系统命令调用 FFmpeg
使用 GStreamer 等多媒体框架
考虑使用 Python 或其他语言的视频处理工具

示例：使用系统命令的 Rust 代码：
System Command MTS to MP4 ConverterClick to open code
这个方案的特点：

依赖系统 FFmpeg
简单直接
性能取决于 FFmpeg

总结建议：

如果追求纯 Rust 解决方案，目前没有完美的库
推荐使用系统命令或 FFmpeg 库
对于跨平台需求，考虑静态链接 FFmpeg


====

mac
brew install ffmpeg


ffmpeg -i input.mts -c copy -bsf:v h264_mp4toannexb output.mp4