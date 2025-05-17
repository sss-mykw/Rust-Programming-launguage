use std::thread;
use std::time::Duration;

fn main() {
    practice_16_1();
}

fn practice_16_1() {
    // spawnで新規スレッドを生成する
    // {
    //     // こちらのスレッドの内容が全て実行されるかどうかはわからない。
    //     // メインスレッドの処理が完了するまで動き続ける
    //     thread::spawn(|| {
    //         for i in 1..10 {
    //             println!("hi number {} from spawned thread!", i);
    //             thread::sleep(Duration::from_millis(1));
    //         }
    //     });
    // 
    //     for i in 1..5 {
    //         println!("hi number {} from main thread!", i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // }
    
    // joinハンドルで全スレッドの終了を待つ
    {
        let handle = thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {} from spawned thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        // この箇所で実行すると、handleの実行が全て実行するまで以降の処理は走らない
        handle.join().unwrap();

        for i in 1..5 {
            println!("hi number {} from main thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
        
        // 指定したハンドルが表すスレッドが終了するまで現在実行中のスレッドのスレッドをブロックする
        // handle.join().unwrap();
    }

    // スレッドでmoveクロージャを使用する
    {
        let v = vec![1, 2, 3];

        let handle = thread::spawn(move || {
            println!("Here's a vector: {:?}", v);
        });
        
        // 所有権が別スレッドに移っているのでコンパイルエラーになる！
        // drop(v);

        handle.join().unwrap();
    }
}
