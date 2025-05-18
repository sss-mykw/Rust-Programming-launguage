mod lib;

use std::cell::RefCell;
use std::ops::Deref;
use std::rc::{Rc, Weak};
use crate::List::{Cons, Nil};

fn main() {
    // practice_15_1();
    // practice_15_2();
    // practice_15_3();
    // practice_15_4();
    // practice_15_5();
    practice_15_6();
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
    // {
    //     let list = Cons(1, Box::new(
    //         Cons(2, Box::new(
    //             Cons(3, Box::new(Nil))
    //         ))
    //     ));
    // }
}

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
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

fn practice_15_3() {
    // {
    //     let c = CustomSmartPointer { data: String::from("my stuff") };
    //     let d = CustomSmartPointer { data: String::from("other stuff") };
    //
    //     println!("CustomSmartPointers created.");
    //     // 変数は生成された順番の逆順でドロップされる
    // }

    // std::mem::dropで早期に値をドロップする
    // Dropトレイトのdropメソッドを手動で呼ぶことは出来ない。
    // スコープが終わる前に値を強制的にドロップさせたい場合、 標準ライブラリが提供するstd::mem::drop関数を呼ぶ必要がある。
    {
        let c = CustomSmartPointer { data: String::from("some data") };
        println!("CustomSmartPointer created.");
        drop(c);
        println!("CustomSmartPointer dropped before the end of main.");
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    // インスタンスがスコープから抜け出す際に呼び出される
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn practice_15_4() {
    // Rc<T>はシングルスレッドで使用されることを想定している
    // Rc<T>でデータを共有する
    // {
    //     let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    //     let b = Cons(3, Rc::clone(&a));
    //     let c = Cons(4, Rc::clone(&a));
    // }
    
    // Rc<T>をクローンすると、参照カウントが増える
    // {
    //     let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    //     // a生成後のカウント = {}
    //     println!("count after creating a = {}", Rc::strong_count(&a));
    //     let b = Cons(3, Rc::clone(&a));
    //     // b生成後のカウント = {}
    //     println!("count after creating b = {}", Rc::strong_count(&a));
    //     {
    //         let c = Cons(4, Rc::clone(&a));
    //         // c生成後のカウント = {}
    //         println!("count after creating c = {}", Rc::strong_count(&a));
    //     }
    //     // cがスコープを抜けた後のカウント = {}
    //     println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    // }
}

fn practice_15_5() {
    // RefCell<T>もシングルスレッドで使用するためのものである

    // Rc<T>とRefCell<T>を組み合わせることで可変なデータに複数の所有者を持たせる
    {
        let value = Rc::new(RefCell::new(5));

        let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

        let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
        let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

        *value.borrow_mut() += 10;

        println!("a after = {:?}", a);
        println!("b after = {:?}", b);
        println!("c after = {:?}", c);
    }
}

fn practice_15_6() {
    // 循環参照を回避する: Rc<T>をWeak<T>に変換する
    {
        let leaf = Rc::new(Node {
            value: 3,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        });
        // upgradeはWeak<T> → Option<Rc<T>> に変換する
        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
        
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });
        
        // downgradeはRc<T> → Weak<T> に変換する。
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    }

    // strong_countとweak_countへの変更を可視化する
    {
        let leaf = Rc::new(Node {
            value: 3,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        });

        println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));

        {
            let branch = Rc::new(Node {
                value: 5,
                parent: RefCell::new(Weak::new()),
                children: RefCell::new(vec![Rc::clone(&leaf)]),
            });

            *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

            println!("branch strong = {}, weak = {}", Rc::strong_count(&branch), Rc::weak_count(&branch));

            println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
        }

        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
        println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
    }
}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}