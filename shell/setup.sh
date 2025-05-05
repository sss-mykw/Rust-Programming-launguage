#!/bin/bash

# エラー時に使う共通関数
exit_with_error() {
  echo "************シェルの実行中断************"
  echo "エラー: $1"
  exit 1
}

./shell/create_branch.sh || exit_with_error "create_branch.sh の実行に失敗しました"

./shell/create_dir.sh || exit_with_error "create_dir.sh の実行に失敗しました"

./shell/add_bin_to_the_toml.sh || exit_with_error "add_bin_to_the_toml.sh の実行に失敗しました"

./shell/copy_run_config.sh || exit_with_error "copy_run_config.sh の実行に失敗しました"

echo "すべての処理が正常に完了しました！"
