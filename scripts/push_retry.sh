#!/bin/bash

REPO_DIR="/root/.openclaw/workspace/mybudy"
MAX_RETRIES=10
RETRY_DELAY=60

echo "开始推送重试..."
echo "时间: $(date)"
echo ""

cd "$REPO_DIR" || exit 1

for i in $(seq 1 $MAX_RETRIES); do
    echo "--- 第 $i 次尝试 ---"
    
    if git push origin main 2>&1; then
        echo "✅ 推送成功！"
        echo "时间: $(date)"
        exit 0
    else
        echo "❌ 推送失败"
        echo "等待 ${RETRY_DELAY} 秒后重试..."
        sleep $RETRY_DELAY
    fi
done

echo ""
echo "达到最大重试次数 ($MAX_RETRIES)，推送失败"
echo "时间: $(date)"
exit 1
