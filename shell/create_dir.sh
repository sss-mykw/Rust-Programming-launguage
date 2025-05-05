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

# 作成するディレクトリのパス
chapter_dir="./src/chapter${chapter_num}"

# すでに存在していたら警告（上書きしない）
if [[ -e "$chapter_dir" ]]; then
  echo "警告: ディレクトリ '$chapter_dir' はすでに存在します。処理を中断します。"
  exit 1
fi

# ディレクトリ作成
mkdir -p "$chapter_dir"

# main.rs ファイル作成
cat <<EOF > "$chapter_dir/main.rs"
fn main() {
    println!("Hello, world!");
}
EOF

echo "chapter${chapter_num} ディレクトリと main.rs を作成しました。"
