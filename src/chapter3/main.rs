use std::io;

// 定数は必ず型を宣言する必要があり、宣言が漏れていると実行時にエラーとなる
const MAX_POINTS: u32 = 100_000;

fn main() {
    practice_3_1();
    println!("-------------------------------------");
    practice_3_2();
}

fn practice_3_1() {
    //
    {
        let mut x = 5;
        println!("The value of x is: {}", x);
        x = 6;
        println!("The value of x is: {}", x);
    }

    // 変数と定数(constants)の違い
    {
        println!("{}", MAX_POINTS);
    }

    // シャドーイング
    {
        let x = 5;

        let x = x + 1;

        {
            let x = x * 2;
            println!("The value of x in the inner scope is: {}", x);
        }

        println!("The value of x is: {}", x);

        // 同じ変数名で違う型に変換出来る
        let spaces = "   ";
        let spaces = spaces.len();
    }
}

fn practice_3_2() {
    // 数値演算
    {
        // 足し算
        let sum = 5 + 10;
        println!("The sum is: {}", sum);

        // 引き算
        let difference = 95.5 - 4.3;
        println!("The difference is: {}", difference);

        // 掛け算
        let product = 4 * 30;
        println!("The product is: {}", product);

        // 割り算
        let quotient = 56.7 / 32.2;
        println!("The quotient is: {}", quotient);
        // 結果は0
        let floored = 2 / 3;
        println!("The floored quotient is: {}", floored);

        // 余り
        let remainder = 43 % 5;
        println!("The remainder is: {}", remainder);
    }

    // 複合型（タプル）
    {
        // 宣言と分解
        let tup = (500, 6.4, 1);
        let (x, y, z) = tup;
        println!("The value of y is: {}", y);

        // 添字アクセス
        println!("{} {} {}", tup.0, tup.1, tup.2);
    }

    // 複合型（配列）
    {
        // サイズは固定長になる
        // ベクタ型を使うことが多いかもしれないが、月のリストなどの固定の要素を用いる際は便利だ
        let a = [1, 2, 3, 4, 5];

        // let a = [3, 3, 3, 3, 3];と同じ
        // let a = [3; 5];

        println!("Please enter an array index.");

        let mut index = String::new();

        io::stdin()
            .read_line(&mut index)
            .expect("Failed to read line");

        let index: usize = index
            .trim()
            .parse()
            .expect("Index entered was not a number");

        let element = a[index];

        println!("The value of the element at index {} is: {}", index, element);
    }
}