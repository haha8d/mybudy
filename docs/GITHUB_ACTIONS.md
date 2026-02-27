# GitHub Actions 自动构建

## 功能

自动构建 Windows、macOS、Linux 三个平台的安装包。

## 触发方式

1. **推送到 main/master 分支** - 自动构建
2. **创建标签** (如 v1.0.0) - 自动构建并发布 Release
3. **手动触发** - 在 Actions 页面点击 "Run workflow"

## 使用方法

### 1. 推送代码到 GitHub

```bash
git init
git add .
git commit -m "Initial commit"
git branch -M main
git remote add origin https://github.com/yourusername/mybudy.git
git push -u origin main
```

### 2. 查看构建状态

- 打开 GitHub 仓库
- 点击 "Actions" 标签
- 查看构建进度

### 3. 下载安装包

构建完成后：
- 点击 Actions 中的构建记录
- 在 "Artifacts" 部分下载安装包

### 4. 发布 Release（可选）

创建标签自动发布：

```bash
git tag v0.1.0
git push origin v0.1.0
```

GitHub 会自动创建 Release，并上传所有安装包。

## 输出文件

| 平台 | 文件 |
|------|------|
| Windows | `.msi`, `.exe` |
| macOS Intel | `.dmg` (x64) |
| macOS Apple Silicon | `.dmg` (arm64) |
| Linux | `.deb`, `.rpm`, `.AppImage` |

## 注意事项

- macOS 构建需要 Apple Developer 账号才能签名
- 未签名的应用打开时会有安全提示
- Windows 构建需要 Visual Studio 工具链（GitHub Actions 已预装）
