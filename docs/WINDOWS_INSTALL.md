# MyBudy Windows 构建和安装指南

## 方法一：使用安装脚本（推荐）

### 1. 准备文件
将以下文件放在同一目录：
- `mybudy.exe` (构建好的程序)
- `install-windows.bat` (安装脚本)
- `resources/` (资源目录，可选)
- `icons/` (图标目录，可选)

### 2. 运行安装脚本
1. 右键点击 `install-windows.bat`
2. 选择"以管理员身份运行"
3. 按提示完成安装

### 3. 安装位置
- 程序：`C:\Program Files\MyBudy\`
- 开始菜单：`MyBudy`
- 桌面快捷方式：`MyBudy`

---

## 方法二：手动构建

### 前提条件
- Windows 10/11
- [Rust](https://rustup.rs/) (最新稳定版)
- [Node.js](https://nodejs.org/) (v18+)
- [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) (C++ 构建工具)

### 构建步骤

```powershell
# 1. 克隆项目
git clone <your-repo-url>
cd mybudy

# 2. 安装前端依赖
npm install

# 3. 安装 Tauri CLI
cargo install tauri-cli

# 4. 构建 Windows 版本
cargo tauri build --target x86_64-pc-windows-msvc
```

构建完成后，安装包在：
`src-tauri/target/release/bundle/msi/MyBudy_0.1.0_x64_en-US.msi`

---

## 方法三：使用 MSI 安装包

如果有 `.msi` 文件：

```powershell
# 双击运行 MSI 文件
# 或命令行安装
msiexec /i MyBudy_0.1.0_x64_en-US.msi /quiet
```

---

## 卸载

### 方法一：使用控制面板
1. 打开"设置" → "应用" → "已安装的应用"
2. 找到 MyBudy
3. 点击"卸载"

### 方法二：手动卸载
```powershell
# 停止程序
taskkill /f /im mybudy.exe

# 删除安装目录
rmdir /s /q "C:\Program Files\MyBudy"

# 删除快捷方式
del "%Public%\Desktop\MyBudy.lnk"
rmdir /s /q "%ProgramData%\Microsoft\Windows\Start Menu\Programs\MyBudy"
```

---

## 常见问题

### Q: 安装时提示"需要管理员权限"
A: 右键点击安装脚本，选择"以管理员身份运行"

### Q: 运行时提示缺少 DLL
A: 安装 [Visual C++ Redistributable](https://aka.ms/vs/17/release/vc_redist.x64.exe)

### Q: 如何更新到新版？
A: 运行安装脚本，选择重新安装即可

### Q: 如何完全清除数据？
A: 卸载后，删除 `%USERPROFILE%\.mybudy\` 目录

---

## 开发模式运行

```powershell
# 安装依赖
npm install

# 开发模式（带热重载）
npm run tauri-dev
```
