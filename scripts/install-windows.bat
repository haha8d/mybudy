@echo off
chcp 65001 >nul
echo ==========================================
echo    MyBudy Windows 安装脚本
echo ==========================================
echo.

:: 检查管理员权限
net session >nul 2>&1
if %errorLevel% neq 0 (
    echo [错误] 请以管理员身份运行此脚本！
    echo 右键点击脚本，选择"以管理员身份运行"
    pause
    exit /b 1
)

echo [1/5] 检查系统要求...

:: 检查 Windows 版本
for /f "tokens=4-5 delims=. " %%i in ('ver') do set VERSION=%%i.%%j
if "%version%" lss "10.0" (
    echo [错误] 需要 Windows 10 或更高版本
    pause
    exit /b 1
)
echo     ✓ Windows 版本检查通过

:: 检查是否已安装
if exist "C:\Program Files\MyBudy\mybudy.exe" (
    echo.
    echo [警告] MyBudy 已安装！
    set /p REINSTALL="是否重新安装? (Y/N): "
    if /i not "%REINSTALL%"=="Y" (
        echo 安装已取消
        pause
        exit /b 0
    )
    echo [2/5] 卸载旧版本...
    taskkill /f /im mybudy.exe 2>nul
    rmdir /s /q "C:\Program Files\MyBudy" 2>nul
    echo     ✓ 旧版本已卸载
) else (
    echo [2/5] 新安装
)

echo.
echo [3/5] 创建安装目录...
if not exist "C:\Program Files\MyBudy" mkdir "C:\Program Files\MyBudy"
echo     ✓ 目录创建成功

echo.
echo [4/5] 复制文件...
:: 假设安装包和此脚本在同一目录
if exist "mybudy.exe" (
    copy /y "mybudy.exe" "C:\Program Files\MyBudy\" >nul
) else (
    echo [错误] 找不到 mybudy.exe！
    echo 请确保 mybudy.exe 与此脚本在同一目录
    pause
    exit /b 1
)

:: 复制其他资源文件
if exist "resources" xcopy /s /y "resources" "C:\Program Files\MyBudy\resources\" >nul 2>&1
if exist "icons" xcopy /s /y "icons" "C:\Program Files\MyBudy\icons\" >nul 2>&1

echo     ✓ 文件复制完成

echo.
echo [5/5] 创建快捷方式...

:: 创建开始菜单快捷方式
if not exist "%ProgramData%\Microsoft\Windows\Start Menu\Programs\MyBudy" (
    mkdir "%ProgramData%\Microsoft\Windows\Start Menu\Programs\MyBudy"
)

powershell -Command "$WshShell = New-Object -comObject WScript.Shell; $Shortcut = $WshShell.CreateShortcut('%ProgramData%\Microsoft\Windows\Start Menu\Programs\MyBudy\MyBudy.lnk'); $Shortcut.TargetPath = 'C:\Program Files\MyBudy\mybudy.exe'; $Shortcut.Save()"

:: 创建桌面快捷方式
powershell -Command "$WshShell = New-Object -comObject WScript.Shell; $Shortcut = $WshShell.CreateShortcut('%Public%\Desktop\MyBudy.lnk'); $Shortcut.TargetPath = 'C:\Program Files\MyBudy\mybudy.exe'; $Shortcut.Save()"

echo     ✓ 快捷方式创建成功

echo.
echo ==========================================
echo    安装完成！
echo ==========================================
echo.
echo 安装位置: C:\Program Files\MyBudy\
echo 开始菜单: MyBudy
echo 桌面快捷方式: MyBudy
echo.
echo 按任意键退出...
pause >nul
