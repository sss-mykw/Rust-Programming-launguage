// 定数は必ず型を宣言する必要があり、宣言が漏れていると実行時にエラーとなる
const MAX_POINTS: u32 = 100_000;

fn main() {
    practice_3_1();
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
