# MyBudy Windows 测试指南

## 1. 下载安装包

1. 打开 https://github.com/haha8d/mybudy/actions
2. 点击最新的工作流运行记录
3. 在 **Artifacts** 部分下载 `mybudy-windows`
4. 解压得到 `.msi` 或 `.exe` 文件

## 2. 安装 MyBudy

### 方式 A：使用 MSI 安装包（推荐）

```powershell
# 双击运行 MyBudy_0.1.0_x64_en-US.msi
# 或命令行安装
msiexec /i MyBudy_0.1.0_x64_en-US.msi /quiet
```

### 方式 B：使用 EXE 安装包

```powershell
# 双击运行 MyBudy_0.1.0_x64-setup.exe
# 按提示完成安装
```

### 方式 C：使用安装脚本

```powershell
# 以管理员身份运行
.\install-windows.bat
```

## 3. 启动 MyBudy

### 方式 1：开始菜单
- 点击开始菜单 → MyBudy

### 方式 2：桌面快捷方式
- 双击桌面上的 MyBudy 图标

### 方式 3：命令行
```powershell
# 如果添加到 PATH
mybudy

# 或直接运行
"C:\Program Files\MyBudy\mybudy.exe"
```

## 4. 测试功能清单

### ✅ 基础功能
- [ ] 悬浮球显示在屏幕右下角
- [ ] 悬浮球可以拖动
- [ ] 点击悬浮球打开主窗口
- [ ] 主窗口显示聊天界面

### ✅ 聊天功能
- [ ] 左侧显示聊天列表
- [ ] 点击 "+" 创建新对话
- [ ] 输入消息并发送
- [ ] 收到 AI 回复（模拟）
- [ ] 删除对话

### ✅ 设置功能
- [ ] 打开设置面板
- [ ] 切换主题（深色/浅色）
- [ ] 配置 AI 模型（Kimi/OpenAI）
- [ ] 修改快捷键

### ✅ 系统功能
- [ ] 截图按钮（框架）
- [ ] 语音按钮（框架）
- [ ] 文件拖拽到悬浮球
- [ ] 系统托盘图标
- [ ] 右键托盘菜单

## 5. 常见问题排查

### 问题 1：无法启动，提示缺少 DLL
**解决：** 安装 Visual C++ Redistributable
```powershell
# 下载并安装
https://aka.ms/vs/17/release/vc_redist.x64.exe
```

### 问题 2：悬浮球不显示
**解决：**
1. 检查是否被其他窗口遮挡
2. 检查系统托盘是否有 MyBudy 图标
3. 重启应用

### 问题 3：无法发送消息
**解决：**
1. 检查是否选择了对话
2. 检查网络连接（实际 AI 接入后需要）
3. 查看控制台错误（F12）

### 问题 4：快捷键无效
**解决：**
1. 检查是否有其他应用占用快捷键
2. 在设置中修改快捷键
3. 重启应用

## 6. 卸载 MyBudy

### 方式 1：控制面板
1. 打开 设置 → 应用 → 已安装的应用
2. 找到 MyBudy
3. 点击 卸载

### 方式 2：命令行
```powershell
# MSI 安装包
msiexec /x MyBudy_0.1.0_x64_en-US.msi /quiet

# 或手动删除
Remove-Item -Recurse -Force "C:\Program Files\MyBudy"
Remove-Item -Recurse -Force "$env:APPDATA\MyBudy"
```

## 7. 反馈问题

如果遇到问题，请提供：
1. Windows 版本（Win10/Win11）
2. 安装包版本
3. 错误截图或错误信息
4. 复现步骤

---

**测试完成后，请勾选上方功能清单，告诉我哪些正常、哪些有问题。**
