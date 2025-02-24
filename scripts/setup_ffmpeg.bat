@echo off
setlocal enabledelayedexpansion

echo 正在检查系统要求...

:: 检查操作系统版本
ver | findstr /i "5\.1\." > nul
if %errorLevel% equ 0 (
    echo 错误：不支持 Windows XP
    pause
    exit /b 1
)

:: 检查管理员权限
net session >nul 2>&1
if %errorLevel% neq 0 (
    echo 错误：请以管理员权限运行此脚本！
    echo 右键点击脚本，选择"以管理员身份运行"
    pause
    exit /b 1
)

:: 检查是否已安装 FFmpeg
where ffmpeg >nul 2>&1
if %errorLevel% equ 0 (
    echo FFmpeg 已安装，检查版本...
    ffmpeg -version
    set /p UPGRADE=是否要重新安装最新版本？(Y/N)
    if /i "!UPGRADE!" neq "Y" (
        echo 安装已取消
        pause
        exit /b 0
    )
)

:: 检查网络连接
ping 8.8.8.8 -n 1 -w 1000 >nul
if %errorLevel% neq 0 (
    echo 警告：网络连接可能不稳定，这可能会影响下载过程
    set /p CONTINUE=是否继续？(Y/N)
    if /i "!CONTINUE!" neq "Y" (
        echo 安装已取消
        pause
        exit /b 1
    )
)

:: 检查磁盘空间
for /f "tokens=3" %%a in ('dir /-c %SystemDrive% ^| find "bytes free"') do set "FREE_SPACE=%%a"
set "FREE_SPACE=%FREE_SPACE:,=%"
if %FREE_SPACE% lss 500000000 (
    echo 错误：磁盘空间不足，至少需要 500MB 可用空间
    pause
    exit /b 1
)

:: 设置变量
set "DOWNLOAD_URL=https://www.gyan.dev/ffmpeg/builds/ffmpeg-release-essentials.zip"
set "FFMPEG_ZIP=%TEMP%\ffmpeg.zip"
set "INSTALL_DIR=%ProgramFiles%\ffmpeg"
set "PATH_TO_ADD=%INSTALL_DIR%\bin"
set "POWERSHELL_VERSION="

:: 检查 PowerShell 版本
for /f "tokens=2 delims=." %%I in ('powershell "$PSVersionTable.PSVersion.Major"') do set "POWERSHELL_VERSION=%%I"
if %errorLevel% neq 0 (
    echo 错误：无法检测 PowerShell 版本
    pause
    exit /b 1
)

echo 正在下载 FFmpeg...
echo 源地址: %DOWNLOAD_URL%
echo 目标位置: %FFMPEG_ZIP%

:: 使用适当的下载方法
if defined POWERSHELL_VERSION (
    powershell -Command "& { $ProgressPreference = 'SilentlyContinue'; Invoke-WebRequest -Uri '%DOWNLOAD_URL%' -OutFile '%FFMPEG_ZIP%' }"
) else (
    bitsadmin /transfer FFmpegDownload /download /priority normal "%DOWNLOAD_URL%" "%FFMPEG_ZIP%"
)

if %errorLevel% neq 0 (
    echo 错误：下载失败！
    echo 请检查网络连接或手动下载：
    echo %DOWNLOAD_URL%
    pause
    exit /b 1
)

echo 正在验证下载...
if not exist "%FFMPEG_ZIP%" (
    echo 错误：下载文件未找到
    pause
    exit /b 1
)

echo 正在创建安装目录...
if exist "%INSTALL_DIR%" (
    echo 清理旧安装...
    rd /s /q "%INSTALL_DIR%"
    if %errorLevel% neq 0 (
        echo 错误：无法删除旧安装
        pause
        exit /b 1
    )
)

mkdir "%INSTALL_DIR%" 2>nul
if %errorLevel% neq 0 (
    echo 错误：无法创建安装目录
    pause
    exit /b 1
)

echo 正在解压 FFmpeg...
powershell -Command "& { $ProgressPreference = 'SilentlyContinue'; Expand-Archive -Path '%FFMPEG_ZIP%' -DestinationPath '%INSTALL_DIR%' -Force }"
if %errorLevel% neq 0 (
    echo 错误：解压失败！
    pause
    exit /b 1
)

:: 移动文件到正确的位置
echo 正在整理文件...
if not exist "%INSTALL_DIR%\bin" mkdir "%INSTALL_DIR%\bin"
for /d %%i in ("%INSTALL_DIR%\ffmpeg-*") do (
    xcopy "%%i\bin\*" "%INSTALL_DIR%\bin\" /E /I /Y
    if %errorLevel% neq 0 (
        echo 错误：无法复制文件
        pause
        exit /b 1
    )
    rd /s /q "%%i"
)

echo 正在设置环境变量...
:: 检查 PATH 中是否已经存在 FFmpeg
set "PATH_FOUND="
for %%i in ("%PATH:;=" "%") do (
    if /i "%%~i"=="%PATH_TO_ADD%" set PATH_FOUND=1
)

if not defined PATH_FOUND (
    setx PATH "%PATH%;%PATH_TO_ADD%" /M
    if %errorLevel% neq 0 (
        echo 错误：设置环境变量失败！
        echo 请手动将以下路径添加到系统环境变量 PATH 中：
        echo %PATH_TO_ADD%
        pause
        exit /b 1
    )
)

:: 清理临时文件
echo 正在清理...
del "%FFMPEG_ZIP%" 2>nul

:: 验证安装
echo 正在验证安装...
where ffmpeg >nul 2>&1
if %errorLevel% neq 0 (
    echo 警告：FFmpeg 未在 PATH 中找到
    echo 请重启命令提示符或 IDE 后重试
) else (
    echo 验证 FFmpeg 功能...
    ffmpeg -version >nul 2>&1
    if %errorLevel% neq 0 (
        echo 警告：FFmpeg 安装可能不完整
    ) else (
        ffmpeg -codecs | find "h264" >nul 2>&1
        if %errorLevel% neq 0 (
            echo 警告：未检测到 H.264 编解码器支持
        )
    )
)

echo.
echo FFmpeg 安装完成！
echo 安装位置: %INSTALL_DIR%
echo.
echo 版本信息:
ffmpeg -version
echo.
echo 请重启命令提示符或 IDE 以使环境变量生效。
echo 如果遇到问题，请确保以下路径已添加到系统环境变量 PATH 中：
echo %PATH_TO_ADD%
echo.
pause
