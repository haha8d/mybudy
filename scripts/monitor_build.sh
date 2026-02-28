#!/bin/bash

REPO="haha8d/mybudy"
WORKFLOW_FILE=".github/workflows/build.yml"

echo "=== MyBudy GitHub Actions 自动修复系统 ==="
echo "开始时间: $(date)"
echo ""

# 获取最新构建状态
get_build_status() {
    curl -s "https://api.github.com/repos/$REPO/actions/runs?per_page=1" | python3 -c "
import json, sys
try:
    data = json.load(sys.stdin)
    if data['workflow_runs']:
        run = data['workflow_runs'][0]
        print(f\"状态: {run['status']}\")
        print(f\"结果: {run['conclusion'] or '进行中'}\")
        print(f\"分支: {run['head_branch']}\")
        print(f\"提交: {run['head_sha'][:7]}\")
        print(f\"消息: {run['display_title']}\")
        print(f\"运行ID: {run['id']}\")
        exit(0 if run['conclusion'] == 'success' else 1)
    else:
        print('没有找到构建记录')
        exit(1)
except Exception as e:
    print(f'错误: {e}')
    exit(1)
"
}

# 获取失败的 jobs
get_failed_jobs() {
    local run_id=$1
    curl -s "https://api.github.com/repos/$REPO/actions/runs/$run_id/jobs" | python3 -c "
import json, sys
try:
    data = json.load(sys.stdin)
    failed = []
    for job in data['jobs']:
        if job['conclusion'] == 'failure':
            failed.append(job['name'])
    print(','.join(failed))
except:
    print('')
"
}

# 主循环
attempt=1
while true; do
    echo "--- 第 $attempt 次尝试 ---"
    
    # 检查最新构建状态
    echo "检查构建状态..."
    if get_build_status; then
        echo "✅ 所有构建成功！"
        exit 0
    fi
    
    # 获取运行ID
    run_id=$(curl -s "https://api.github.com/repos/$REPO/actions/runs?per_page=1" | python3 -c "import json,sys; d=json.load(sys.stdin); print(d['workflow_runs'][0]['id'] if d['workflow_runs'] else '')")
    
    if [ -n "$run_id" ]; then
        echo "运行ID: $run_id"
        failed_jobs=$(get_failed_jobs $run_id)
        if [ -n "$failed_jobs" ]; then
            echo "❌ 失败的任务: $failed_jobs"
        fi
    fi
    
    echo ""
    echo "等待30分钟后再次检查..."
    echo "下次检查时间: $(date -d '+30 minutes' 2>/dev/null || date -v+30M)"
    echo ""
    
    sleep 1800  # 30分钟
    ((attempt++))
done
