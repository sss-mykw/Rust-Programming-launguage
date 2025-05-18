use std::ops::Deref;
use crate::List::{Cons, Nil};

fn main() {
    // practice_15_1();
    practice_15_2();
}

fn practice_15_1() {
    // Box<T>を使ってヒープにデータを確保する
    {
        // Boxを使うことで値5はスタックではなくヒープに確保される
        let b = Box::new(5);
        println!("b = {}", b);
        // スコープを抜ける際にスタックに格納されているBoxと、そのBoxが指し示すヒープに確保されている値5に対して実施される
    }
    
    // ボックスで再帰的な型を可能にする
    {
        let list = Cons(1, Box::new(
            Cons(2, Box::new(
                Cons(3, Box::new(Nil))
            ))
        ));
    }
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn practice_15_2() {
    // 参照外し演算子で値までポインタを追いかける
    {
        let x = 5;
        let y = &x;
        
        assert_eq!(5, x);
        // yはxへのポインタなので、参照が指している値を取得するには参照外しが必要になる
        assert_eq!(5, *y);
    }

    // Box<T>を参照のように使う
    {
        let x = 5;
        let y = Box::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }
    
    // 独自のスマートポインタを定義する
    // Derefトレイトを実装して型を参照のように扱う
    {
        let x = 5;
        let y = MyBox::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    // 関数やメソッドで暗黙的な参照外し型強制
    {
        let m = MyBox::new(String::from("Rust"));
        hello(&m);
        // 参照外し型強制（Deref）がなかった場合は以下のような難しい記述になる
        hello(&(*m)[..]);
    }
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}