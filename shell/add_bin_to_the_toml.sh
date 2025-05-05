#!/bin/bash

# 現在のブランチ名を取得
branch_name=$(git branch --show-current)

# ブランチ名が ChapterX の形式かチェック
if [[ "$branch_name" =~ ^Chapter([0-9]+)$ ]]; then
  chapter_num="${BASH_REMATCH[1]}"
else
  echo "エラー: ブランチ名が 'chapterX' の形式ではありません（期待するブランチ形式の例: chapter3）"
  exit 1
fi

# Cargo.toml のパス（実行ディレクトリ直下を想定）
cargo_toml="./Cargo.toml"

# 追記内容を作成
entry=$(cat <<EOF
[[bin]]
name = "chap$chapter_num"
path = "src/chapter$chapter_num/main.rs"

EOF
)

# Cargo.toml に追記
echo "$entry" >> "$cargo_toml"

echo "Cargo.toml にエントリを追加しました: chap$chapter_num"
