fn main() {
    practice_8_1()
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
