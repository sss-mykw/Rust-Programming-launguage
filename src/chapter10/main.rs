fn main() {
    practice_10_1()
}

fn practice_10_1() {
    // 構造体定義
    {
        let point = Point { x: 5, y: 10 };
        let point = Point { x: 1.0, y: 4.0 };
    }
    
    // enum定義
    // OptionやResult型など
    
    // メソッド定義
    {
        let point = Point { x: 5, y: 10 };
        println!("x: {}", point.x());
    }
}

struct Point<T> {
    x: T,
    y: T,
}

impl <T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
