fn main() {
    // practice_5_1();
    practice_5_2_and_5_3();
}

fn practice_5_1() {
    let user1 = build_user(String::from("<EMAIL>"), String::from("someusername123"));
    
    // mutを付けるとフィールドが可変になる
    let mut user2 = build_user(String::from("<EMAIL>"), String::from("someusername123"));
    user2.email = String::from("<EMAIL>hoge");
    
    // 構造体更新記法
    let user3 = User {
        email: String::from("<EMAIL>"),
        username: String::from("someusername123"),
        ..user1
    };
}

fn build_user(email: String, username: String) -> User {
    // フィールド初期化省略記法
    // メソッドの引数とフィールド名が同一の場合に使用可能
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn practice_5_2_and_5_3() {
    // メソッドの定義やデバッグ情報の出力
    {
        let rectangle = Rectangle {
            width: 30,
            height: 50,
        };
        println!("The area of the rectangle is {} square pixels.", rectangle.area());
        println!("{:?}", rectangle);
        println!("{:#?}", rectangle);
    }

    // 引数の多いメソッド
    {
        let rect1 = Rectangle { width: 30, height: 50 };
        let rect2 = Rectangle { width: 10, height: 40 };
        let rect3 = Rectangle { width: 60, height: 45 };

        println!("Can rect1 hold rect2 ? : {}", rect1.can_hold(&rect2));
        println!("Can rect1 hold rect3 ? : {}", rect1.can_hold(&rect3));
    }

    // 関連関数
    {
        let square = Rectangle::square(30);
        println!("The area of the square is {} square pixels.", square.area());
        println!("{:?}", square);
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    // otherはreadしかしないので不変借用となる
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// implを分けることも出来る
impl Rectangle {
    // 関連関数
    // 引数にselfを取らないが、対象の構造体と関連がある場合に使用する
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
