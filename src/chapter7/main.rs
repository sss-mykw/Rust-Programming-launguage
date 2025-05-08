// 構造体やenumなどの要素を持ち込む際はフルパス指定が慣例
use std::collections::HashMap; // 標準ライブラリクレートの名前stdから始まる

// asで別名を名付けることが可能
use std::io::Result as IoResult;

// 巨大なuseのリストをネストしたパスを使って整理する
// パスの共通部分を持つ場合は以下のようにまとめて書けるので、縦長になることを防止できる！
use std::{cmp::Ordering, io};

// use std::io;
// use std::io::Write;
// 上記２つを合体させると以下
// use std::io::{self, Write};

// globe演算子（ワイルドカード）
// use std::collections::*;

fn main() {
    println!("Hello, world!");

    // 7.4
    {
        let mut map = HashMap::new();
        map.insert(1, 2);
    }
}
