use std::collections::HashMap;

fn main() {
    // practice_8_1();
    // practice_8_2();
    practice_8_3();
}

fn practice_8_1() {
    // 新しいベクタを生成する
    {
        let v = vec![1, 2, 3];
    }

    // ベクタを更新する
    {
        let mut v: Vec<i32> = Vec::new();

        v.push(5);
        v.push(6);
        v.push(7);
        v.push(8);
    }

    // ベクタの要素を読む
    {
        let v = vec![1, 2, 3, 4, 5];

        let third = &v[2];
        println!("The third element is {}", third);

        match v.get(2) {
            None => {
                println!("There is no third element.");
            }
            Some(third) => {
                println!("The third element is {}", third);
            }
        }

        // この記法はpanicになる
        // let does_not_exist = &v[100];
        // 一歩でこちらの記法ではpanicとならず、Noneとなる
        let does_not_exist = v.get(100);
        println!("{:?}", does_not_exist);

        // 不変借用が実行されているので可変借用は出来ない。
        // 新たな要素をベクタの終端に追加する際に、ベクタのある場所に全要素を隣り合わせに配置するだけのスペースがない場合、
        // 新しいメモリを割り当て、古い要素を新しいスペースにコピーする必要がある。
        // その場合に最初の要素を指す参照は解放されたメモリを指すことになり、意図しない実装になってしまう。
        // 借用規則がそのような状況に陥らないように防いでくれる。
        // v.push(6);
    }

    // ベクタ内の値を順に処理する
    {
        let v = vec![1, 2, 3, 4, 5];

        for i in &v {
            println!("{}", i);
        }

        let mut v = vec![1, 2, 3, 4, 5];

        for i in &mut v {
            // *は参照外し演算子と呼ばれる
            *i += 50;
        }

        println!("{:?}", v);
    }

    // Enumを使って複数の型を保持する
    {
        enum SpreadsheetCell {
            Int(i32),
            Float(f64),
            Text(String),
        }

        let row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Text(String::from("blue")),
            SpreadsheetCell::Float(10.12),
        ];
    }
}

fn practice_8_2() {
    // 新規文字列を生成する
    {
        let data = "initial contents";
        let s = data.to_string();
        let s = String::from("initial contents");
    }

    // 文字列を更新する
    {
        let mut s = String::from("foo");
        // 追加対象が文字列（&str）の場合はpush_strを用いる
        s.push_str("bar");
        // 追加対象が単一の文字（char）の場合はpushを用いる
        s.push('z');
    }

    // +（add）による文字列連結
    {
        let s1 = String::from("Hello, ");
        let s2 = String::from("world!");
        // s1（文字列連結のベース）はmoveされるが、s2は参照を用いるのでmoveされない（所有権）
        let s3 = s1 + &s2;
        println!("{}", s3);
        // println!("{}", s1);
        println!("{}", s2);
    }

    // format!マクロを用いた文字列連結
    // Memo 連結というより、それぞれの{}の箇所で参照しているという理解のほうが正しそう
    {
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");
        let s = format!("{}-{}-{}", s1, s2, s3);
        println!("{}", s);
        println!("{}", s1);
        println!("{}", s2);
        println!("{}", s3);
    }
}

fn practice_8_3() {
    // 新規ハッシュマップを生成する
    {
        let mut scores = HashMap::new();
        
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);
    }

    {
        let teams = vec![String::from("Blue"), String::from("Yellow")];
        let initial_scores = vec![10, 50];
        
        // _はベクタの型を用いて推論させるために明示しないといけない
        let mut scores: HashMap<_, _> =
            teams.iter().zip(initial_scores.iter()).collect();
        
        println!("{:?}", scores);
    }
    
    // ハッシュマップと所有権
    {
        let field_name = String::from("Favorite color");
        let field_value = String::from("Blue");
        let mut map = HashMap::new();
        map.insert(field_name, field_value);
        // インサートした値は所有権が奪われるので使用出来なくなる
        // field_name
        
        let mut map = HashMap::new();
        let not_removed_key = String::from("hoge");
        let not_removed_value = String::from("fuga");
        // 値への参照をハッシュマップに入れた場合は、所有権が移らない
        map.insert(&not_removed_key, &not_removed_value);
        println!("{:?}", map);
        println!("{:?}", not_removed_value);
    }

    // ハッシュマップの値にアクセスする
    {
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);
        
        for (key, value) in &scores {
            //　挿入順ではないことに注意すること
            println!("{}: {}", key, value);
        }
    }
    
    // ハッシュマップを更新する
    {
        // 値を上書きするケース
        {
            let mut scores = HashMap::new();
            scores.insert(String::from("Blue"), 10);
            scores.insert(String::from("Blue"), 25);
            // memo 直感通り新しい値に上書きされて25になる
            println!("{:?}", scores);
        }
        
        // キーに値がなかった時のみ値を挿入する
        {
            let mut scores = HashMap::new();
            scores.insert(String::from("Blue"), 10);
            
            scores.entry(String::from("Yellow")).or_insert(50);
            scores.entry(String::from("Blue")).or_insert(50);
            
            println!("{:?}", scores);
        }
        
        // 古い値に基づいて値を更新する
        {
            let text = "hello world wonderful world";
            let mut map = HashMap::new();
            
            for word in text.split_whitespace() {
                let count = map.entry(word).or_insert(0);
                // 参照外しを忘れずに
                *count += 1;
            }
            
            println!("{:?}", map);
        }
    }
}
