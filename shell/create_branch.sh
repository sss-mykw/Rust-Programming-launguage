#!/bin/bash

# 作業ディレクトリの状態をチェック
if [[ -n "$(git status --porcelain)" ]]; then
  echo "未コミットまたは未ステージのファイルがあります。作業を中断します。"
  exit 1
fi

# main ブランチに切り替え
echo "main ブランチに切り替えます..."
git checkout main || { echo "main ブランチに切り替えできませんでした。"; exit 1; }

# pull を実行
echo "最新の状態を取得します..."
git pull origin main || { echo "pull に失敗しました。"; exit 1; }

# 章番号の入力をループで受け取る
while true; do
  read -p "第何章用のブランチですか？章の番号を入力してください: " chapter_num

  # 入力が数値かどうかを判定
  if [[ "$chapter_num" =~ ^[0-9]+$ ]]; then
    break  # 正しい数値ならループを抜ける
  else
    echo "無効な入力です。半角の数値を入力してください。"
  fi
done

branch_name="chapter$chapter_num"

# 同名のローカルブランチがすでに存在していないか確認
if git show-ref --verify --quiet "refs/heads/$branch_name"; then
  echo "エラー: ブランチ '$branch_name' はすでに存在します。"
  exit 1
fi

# 新しいブランチを作成して切り替え
echo "新しいブランチ '$branch_name' を作成して切り替えます..."
git checkout -b "$branch_name" || { echo "ブランチ作成に失敗しました。"; exit 1; }

echo "完了しました。現在のブランチは $(git branch --show-current) です。"
