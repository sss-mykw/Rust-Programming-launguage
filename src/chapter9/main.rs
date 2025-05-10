use std::fs::File;
use std::io::{Error, Read};

fn main() {
    // practice_9_1()
    practice_9_2()
}

fn practice_9_1() {
    let v = vec![1, 2, 3];

    v[99];
    
    // 実行時に環境変数RUST_BACKTRACEをfullに指定すると詳細なトレースが表示される
}

fn practice_9_2() {
    // 色々なエラーにマッチする
    // {
    //     let f = File::open("hello.txt");
    // 
    //     let f = match f {
    //         Ok(file) => file,
    //         // Memo ファイル作成したくないのでコメントアウト
    //         // Err(ref error) if error.kind() == std::io::ErrorKind::NotFound => {
    //         //     match File::create("hello.txt") {
    //         //         Ok(fc) => fc,
    //         //         Err(error) => panic!("There was a problem creating the file: {:?}", error),
    //         //     }
    //         // }
    //         Err(error) => {
    //             panic!("There was a problem opening the file: {:?}", error);
    //         }
    //     };
    // }

    // unwrapとexpect
    // {
    //     // let f = File::open("hello.txt").unwrap();
    //     // エラーメッセージをカスタマイズする
    //     let f = File::open("hello.txt").expect("Failed to open hello.txt");
    // }
    
    // エラーを委譲する
    {
        let user_name = read_username_from_file();
        match user_name {
            Ok(result) => println!("Hello, {}", result),
            Err(error) => println!("Error: {}", error),
        }
    }

    // ?演算子は、Resultを返す関数でしか使用できない
    {
        // let f = File::open("hello.txt")?;
    }
}

fn read_username_from_file() -> Result<String, std::io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(& mut s)?;
    Ok(s)
}
