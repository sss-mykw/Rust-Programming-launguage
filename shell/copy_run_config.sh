#!/bin/bash

# 現在のブランチ名を取得
branch_name=$(git branch --show-current)

# ブランチ名が "chapterX" 形式であることを確認
if [[ "$branch_name" =~ ^chapter([0-9]+)$ ]]; then
  chapter_num="${BASH_REMATCH[1]}"
else
  echo "エラー: ブランチ名が 'chapterX' の形式ではありません（期待するブランチ形式の例: chapter3）"
  exit 1
fi

# 実行コマンドを作成
run_cmd="run --package rust-programming-language --bin chap$chapter_num"

# コマンドをクリップボードにコピー（macOS: pbcopy）
echo "$run_cmd" | pbcopy
echo "以下のコマンドをクリップボードにコピーしました:"
echo "$run_cmd"
