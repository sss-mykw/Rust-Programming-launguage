fn main() {
    // practice_6_1();
    practice_6_2();
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

fn practice_6_2() {
    {
        let penny = Coin::Penny;
        println!("{}", value_in_cents(penny))
    }

    // 値に束縛されるパターン
    {
        let alaska = UsState::Alaska;
        let quarter = Coin::Quarter(alaska);
        println!("{}", value_in_cents(quarter))   
    }

    // Option<T>とのマッチ
    {
        let five = Some(5);
        let six = plus_one(five);
        let none = plus_one(None);
        println!("{:?}, {:?}", five, six);
        println!("{:?}", none);   
    }

    // _というプレースホルダー
    {
        let some_u8_value = 0u8;
        match some_u8_value {
            1 => println!("one"),
            3 => println!("three"),
            5 => println!("five"),
            7 => println!("seven"),
            _ => (),
        }
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

#[derive(Debug)] // すぐに州を点検できるように
enum UsState {
    Alabama,
    Alaska,
    // ... などなど
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
