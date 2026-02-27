#!/bin/bash

# MyBudy Test Script

set -e

echo "=== MyBudy Test Suite ==="

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

print_info() {
    echo -e "${GREEN}[TEST]${NC} $1"
}

print_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

print_error() {
    echo -e "${RED}[FAIL]${NC} $1"
}

print_pass() {
    echo -e "${GREEN}[PASS]${NC} $1"
}

print_section() {
    echo -e "\n${BLUE}=== $1 ===${NC}"
}

# Test counter
TESTS_PASSED=0
TESTS_FAILED=0

# Run a test
run_test() {
    local test_name=$1
    local test_cmd=$2
    
    print_info "Running: $test_name"
    
    if eval "$test_cmd" > /dev/null 2>&1; then
        print_pass "$test_name"
        TESTS_PASSED=$((TESTS_PASSED + 1))
        return 0
    else
        print_error "$test_name"
        TESTS_FAILED=$((TESTS_FAILED + 1))
        return 1
    fi
}

# Test 1: Check directory structure
test_directory_structure() {
    print_section "Directory Structure Tests"
    
    run_test "src-tauri directory exists" "test -d /root/.openclaw/workspace/mybudy/src-tauri"
    run_test "src directory exists" "test -d /root/.openclaw/workspace/mybudy/src"
    run_test "scripts directory exists" "test -d /root/.openclaw/workspace/mybudy/scripts"
    run_test "docs directory exists" "test -d /root/.openclaw/workspace/mybudy/docs"
}

# Test 2: Check Rust files
test_rust_files() {
    print_section "Rust Source Tests"
    
    run_test "main.rs exists" "test -f /root/.openclaw/workspace/mybudy/src-tauri/src/main.rs"
    run_test "Cargo.toml exists" "test -f /root/.openclaw/workspace/mybudy/src-tauri/Cargo.toml"
    run_test "tauri.conf.json exists" "test -f /root/.openclaw/workspace/mybudy/src-tauri/tauri.conf.json"
    run_test "commands module exists" "test -d /root/.openclaw/workspace/mybudy/src-tauri/src/commands"
    run_test "config module exists" "test -d /root/.openclaw/workspace/mybudy/src-tauri/src/config"
    run_test "db module exists" "test -d /root/.openclaw/workspace/mybudy/src-tauri/src/db"
}

# Test 3: Check frontend files
test_frontend_files() {
    print_section "Frontend Tests"
    
    run_test "index.html exists" "test -f /root/.openclaw/workspace/mybudy/index.html"
    run_test "float.html exists" "test -f /root/.openclaw/workspace/mybudy/float.html"
    run_test "package.json exists" "test -f /root/.openclaw/workspace/mybudy/package.json"
    run_test "vite.config.js exists" "test -f /root/.openclaw/workspace/mybudy/vite.config.js"
    run_test "main.js exists" "test -f /root/.openclaw/workspace/mybudy/src/main.js"
    run_test "float.js exists" "test -f /root/.openclaw/workspace/mybudy/src/float.js"
}

# Test 4: Check configuration files
test_config_files() {
    print_section "Configuration Tests"
    
    run_test "config module exists" "test -f /root/.openclaw/workspace/mybudy/src-tauri/src/config/mod.rs"
    run_test "db migrations exist" "test -f /root/.openclaw/workspace/mybudy/src-tauri/migrations/001_initial.sql"
}

# Test 5: Check styles
test_styles() {
    print_section "Style Tests"
    
    run_test "main.css exists" "test -f /root/.openclaw/workspace/mybudy/src/styles/main.css"
    run_test "float.css exists" "test -f /root/.openclaw/workspace/mybudy/src/styles/float.css"
}

# Test 6: Check build script
test_build_script() {
    print_section "Build Script Tests"
    
    run_test "build.sh exists" "test -f /root/.openclaw/workspace/mybudy/scripts/build.sh"
    run_test "build.sh is executable" "test -x /root/.openclaw/workspace/mybudy/scripts/build.sh"
}

# Test 7: Syntax validation
test_syntax() {
    print_section "Syntax Validation"
    
    # Check JSON files
    run_test "tauri.conf.json is valid JSON" "cat /root/.openclaw/workspace/mybudy/src-tauri/tauri.conf.json | python3 -m json.tool > /dev/null"
    run_test "package.json is valid JSON" "cat /root/.openclaw/workspace/mybudy/package.json | python3 -m json.tool > /dev/null"
}

# Test 8: Check components
test_components() {
    print_section "Component Tests"
    
    run_test "App.js exists" "test -f /root/.openclaw/workspace/mybudy/src/components/App.js"
    run_test "Sidebar.js exists" "test -f /root/.openclaw/workspace/mybudy/src/components/Sidebar.js"
    run_test "ChatArea.js exists" "test -f /root/.openclaw/workspace/mybudy/src/components/ChatArea.js"
}

# Test 9: Check stores
test_stores() {
    print_section "Store Tests"
    
    run_test "chatStore.js exists" "test -f /root/.openclaw/workspace/mybudy/src/stores/chatStore.js"
    run_test "configStore.js exists" "test -f /root/.openclaw/workspace/mybudy/src/stores/configStore.js"
}

# Test 10: Cargo.toml validation
test_cargo_toml() {
    print_section "Cargo.toml Validation"
    
    run_test "Cargo.toml has package name" "grep -q 'name = \"mybudy\"' /root/.openclaw/workspace/mybudy/src-tauri/Cargo.toml"
    run_test "Cargo.toml has tauri dependency" "grep -q 'tauri' /root/.openclaw/workspace/mybudy/src-tauri/Cargo.toml"
    run_test "Cargo.toml has serde dependency" "grep -q 'serde' /root/.openclaw/workspace/mybudy/src-tauri/Cargo.toml"
}

# Print summary
print_summary() {
    echo ""
    print_section "Test Summary"
    echo -e "Tests Passed: ${GREEN}$TESTS_PASSED${NC}"
    echo -e "Tests Failed: ${RED}$TESTS_FAILED${NC}"
    echo -e "Total Tests: $((TESTS_PASSED + TESTS_FAILED))"
    
    if [ $TESTS_FAILED -eq 0 ]; then
        echo -e "\n${GREEN}All tests passed!${NC}"
        exit 0
    else
        echo -e "\n${RED}Some tests failed!${NC}"
        exit 1
    fi
}

# Main
main() {
    test_directory_structure
    test_rust_files
    test_frontend_files
    test_config_files
    test_styles
    test_build_script
    test_syntax
    test_components
    test_stores
    test_cargo_toml
    
    print_summary
}

main "$@"
