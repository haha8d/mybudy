fn main() {
    // Windows 资源由 Tauri 自动处理，无需手动设置
    // 避免与 tauri-winres 的版本资源冲突
    
    tauri_build::build()
}
