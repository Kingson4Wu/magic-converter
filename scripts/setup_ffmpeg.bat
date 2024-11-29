@echo off
setlocal enabledelayedexpansion

:: 检查管理员权限
net session >nul 2>&1
if %errorLevel% neq 0 (
    echo 请以管理员权限运行此脚本！
    echo 右键点击脚本，选择"以管理员身份运行"
    pause
    exit /b 1
)

:: 设置变量
set "DOWNLOAD_URL=https://www.gyan.dev/ffmpeg/builds/ffmpeg-release-essentials.zip"
set "FFMPEG_ZIP=%TEMP%\ffmpeg.zip"
set "INSTALL_DIR=%ProgramFiles%\ffmpeg"
set "PATH_TO_ADD=%INSTALL_DIR%\bin"

echo 正在下载 FFmpeg...
powershell -Command "& {Invoke-WebRequest -Uri '%DOWNLOAD_URL%' -OutFile '%FFMPEG_ZIP%'}"
if %errorLevel% neq 0 (
    echo 下载失败！
    pause
    exit /b 1
)

echo 正在创建安装目录...
if not exist "%INSTALL_DIR%" mkdir "%INSTALL_DIR%"

echo 正在解压 FFmpeg...
powershell -Command "& {Expand-Archive -Path '%FFMPEG_ZIP%' -DestinationPath '%INSTALL_DIR%' -Force}"
if %errorLevel% neq 0 (
    echo 解压失败！
    pause
    exit /b 1
)

:: 移动文件到正确的位置
for /d %%i in ("%INSTALL_DIR%\ffmpeg-*") do (
    xcopy "%%i\bin\*" "%INSTALL_DIR%\bin\" /E /I /Y
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
        echo 设置环境变量失败！
        pause
        exit /b 1
    )
)

:: 清理临时文件
del "%FFMPEG_ZIP%"

echo.
echo FFmpeg 安装完成！
echo 安装位置: %INSTALL_DIR%
echo.
echo 请重启命令提示符或 IDE 以使环境变量生效。
echo.
ffmpeg -version
pause
