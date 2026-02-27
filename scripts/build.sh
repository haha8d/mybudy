#!/bin/bash
# MyBudy 跨平台构建脚本

set -e

echo "================================"
echo "   MyBudy 构建脚本"
echo "================================"
echo ""

# 检测操作系统
OS=$(uname -s)
ARCH=$(uname -m)

echo "检测到系统: $OS $ARCH"

# 检查依赖
check_deps() {
    echo "[1/3] 检查依赖..."
    
    if ! command -v node &> /dev/null; then
        echo "错误: 未安装 Node.js"
        echo "请访问 https://nodejs.org/ 安装"
        exit 1
    fi
    
    if ! command -v cargo &> /dev/null; then
        echo "错误: 未安装 Rust"
        echo "请运行: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
        exit 1
    fi
    
    if ! command -v cargo-tauri &> /dev/null; then
        echo "安装 Tauri CLI..."
        cargo install tauri-cli
    fi
    
    echo "    ✓ 依赖检查通过"
}

# 安装 npm 依赖
install_deps() {
    echo ""
    echo "[2/3] 安装 npm 依赖..."
    npm install
    echo "    ✓ npm 依赖安装完成"
}

# 构建
build() {
    echo ""
    echo "[3/3] 构建 $OS 版本..."
    
    case "$OS" in
        Linux)
            cargo tauri build
            echo ""
            echo "构建完成！"
            echo "安装包位置:"
            ls -lh src-tauri/target/release/bundle/*/*.{deb,rpm,AppImage} 2>/dev/null || true
            ;;
        Darwin)
            cargo tauri build
            echo ""
            echo "构建完成！"
            echo "安装包位置:"
            ls -lh src-tauri/target/release/bundle/dmg/*.dmg 2>/dev/null || true
            ;;
        MINGW*|CYGWIN*|MSYS*)
            cargo tauri build --target x86_64-pc-windows-msvc
            echo ""
            echo "构建完成！"
            echo "安装包位置:"
            ls -lh src-tauri/target/release/bundle/msi/*.msi 2>/dev/null || true
            ;;
        *)
            echo "不支持的操作系统: $OS"
            exit 1
            ;;
    esac
}

# 主流程
main() {
    check_deps
    install_deps
    build
    
    echo ""
    echo "================================"
    echo "   构建成功！"
    echo "================================"
}

main "$@"
