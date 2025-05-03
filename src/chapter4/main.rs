fn main() {
    // practice_4_1();
    // practice_4_2();
    practice_4_3();
}

fn practice_4_1() {
    // 変数とデータの相互作用法: ムーブ
    {
        let s1 = String::from("hello");
        let s2 = s1;

        // 下記はエラーになる。s1はs2にムーブされたのでs1は使用できない
        // println!("{}, world!", s1);
    }
    
    // 変数とデータの相互作用法: クローン
    {
        let s1 = String::from("hello");
        // ヒープデータが実際にコピーさせる
        let s2 = s1.clone();

        println!("s1 = {}, s2 = {}", s1, s2);
    }
    
    // スタックのみのデータ: コピー
    {
        let x = 5;
        let y = x;

        // スカラー型はcopy型である。
        println!("x = {}, y = {}", x, y);
    }
    
    // 所有権と関数
    {
        let s1 = String::from("hello");
        let (s2, len) = calculate_length_4_1(s1);
        
        println!("The length of '{}' is {}.", s2, len);
    }
}

fn calculate_length_4_1(s: String) -> (String, usize) {
    let length = s.len();

    // 参照を使っていないので冗長な書き方になっている
    (s, length)
}

fn practice_4_2() {
    // 参照と借用
    {
        let s1 = String::from("hello");
        let len = calculate_length_4_2(&s1);

        println!("The length of '{}' is {}.", s1, len);
    }

    // 可能な参照
    {
        let mut s = String::from("hello");

        change(&mut s);
    }

    // 可変な参照の制限
    {
        let mut s = String::from("hello");

        let r1 = &mut s;
        // 可変な参照は一つしか持てない
        // let r2 = &mut s;
        // 
        // println!("{}, {}", r1, r2);
    }
    
    // データ競合が起こる条件
    // ・2つ以上のポインタが同じデータに同時にアクセスする。
    // ・少なくとも一つのポインタがデータに書き込みを行っている。
    // ・データへのアクセスを同期する機構が使用されていない。

    // 可変と不変な参照を組み合わせ
    {
        let mut s = String::from("hello");

        let r1 = &s; // 問題なし
        let r2 = &s; // 問題なし
        // 問題あり
        // 不変な参照があるのに、それを可変にされてしまったら予期せぬ問題が生じる可能性があるので許されていない
        let r3 = &mut s;
        
        // println!("{}, {}, {}", r1, r2, r3);
    }
    
    // ダングリングポインタ
    // 他人に渡されてしまった可能性のあるメモリを指すポインタのことであり、
    // その箇所へのポインタを保持している間にメモリを解放してしまうことで発生する
    {
        let reference_to_nothing = no_dangle();
    }
    
    // 参照の規則
    // ・任意のタイミングで、「一つの可変参照」または「不変な参照をいくつでも」のどちらかを行える。
    // ・参照は常に有効でなければならない。
}

// 関数の引数に参照を取ることを”借用”と呼ぶ
fn calculate_length_4_2(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// 問題点（２）ライフタイムの指定がない（ライフタイムについては後続の章で学習）
// fn dangle() -> &String {
//     let s = String::from("hello");
// 
//     // 問題点（１）sの生存範囲がこのスコープ内にも関わらず、その参照を返そうとしてしまっている
//     &s
// }

fn no_dangle() -> String {
    let s = String::from("hello");
    
    // 所有権がムーブされる
    s
}

fn practice_4_3() {
    // String型
    let my_string = String::from("hello world");
    let word = first_word(&my_string[..]);
    println!("The first word is {}", word);

    // &'static str 型
    let my_string_literal = "hello world";

    let word = first_word(&my_string_literal[..]);
    println!("The first word is {}", word);

    // 文字列リテラルは「それ自体すでに文字列スライスなので」、スライス記法なしでも機能するのだ！
    let word = first_word(my_string_literal);
    println!("The first word is {}", word);
}

// &Stringではなく&strにすると柔軟性が上がる
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
