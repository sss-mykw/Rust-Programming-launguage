fn main() {
    practice_4_1();
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
        let (s2, len) = calculate_length(s1);
        
        println!("The length of '{}' is {}.", s2, len);
    }
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    // 参照を使っていないので冗長な書き方になっている
    (s, length)
}
