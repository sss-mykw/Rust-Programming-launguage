mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("add_to_waitlist");
        }

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

fn serve_order() {
    println!("serve_order");
}

mod back_of_house {
    pub fn fix_incorrect_order() {
        cook_order();
        // 親を探索
        // → 1つ上の階層はfront_of_house、back_of_house、serve_orderが定義されている
        // → serve_orderが見つかる
        super::serve_order();
    }

    fn cook_order() {
        println!("cook_order");
    }

    pub struct Breakfast {
        // toastだけ公開する
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // enum定義の前にpubが置かれてると、すべての列挙子が公開される
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    // パスをpubキーワードで公開する
    {
        // 絶対パス
        crate::front_of_house::hosting::add_to_waitlist();

        // 相対パス
        front_of_house::hosting::add_to_waitlist();
    }

    // 相対パスをsuperで始める
    {
        back_of_house::fix_incorrect_order();
    }

    // 構造体とenumを公開する
    {
        // 夏 (summer) にライ麦 (Rye) パン付き朝食を注文
        let mut meal = back_of_house::Breakfast::summer("Rye");
        // やっぱり別のパンにする
        meal.toast = String::from("Wheat");
        println!("I'd like {} toast please", meal.toast);

        // seasonal_fruitは公開されていないので、下の行はコンパイルできない。
        // meal.seasonal_fruit = String::from("blueberries");

        // seasonal_fruitは公開されていないので、以下のようなインスタンス生成は出来ない
        // そのためインスタンス生成用の関連関数summerを用意している
        // let mut meal = crate::back_of_house::Breakfast {
        //     toast: String::from("hoge"),
        //     seasonal_fruit: String::from("fuga")
        // };

        let order1 = back_of_house::Appetizer::Soup;
        let order2 = back_of_house::Appetizer::Salad;
    }
}
