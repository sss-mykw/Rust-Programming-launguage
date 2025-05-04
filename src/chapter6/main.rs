fn main() {
    practice_6_1();
}

fn practice_6_1() {
    // enumの値
    {
        let home = IpAddr::V4(127, 0, 0, 1);
        let loopback = IpAddr::V6(String::from("::1"));
    }

    {
        let m = Message::Write(String::from("hello"));
        m.call();
    }

    // Option enumとNull値に勝る利点
    {
        let x: i8 = 5;
        let y: Option<i8> = Some(5);

        // i8とOption<i8>は型が違うのでそのままでは計算出来ない
        // let sum = x + y;
    }
}

enum IpAddr {
    // 列挙子毎に紐付けるデータの型を変更出来る
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => {
                println!("Quit");
            }
            Message::Move { .. } => {
                println!("Move");
            }
            Message::Write(word) => {
                println!("Write {}", word);   
            }
            Message::ChangeColor(_, _, _) => {
                println!("ChangeColor");  
            }
        }
    }
}