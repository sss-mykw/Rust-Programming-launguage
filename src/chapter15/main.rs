use crate::List::{Cons, Nil};

fn main() {
    practice_15_1();
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