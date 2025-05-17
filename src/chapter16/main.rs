use std::rc::Rc;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    // practice_16_1();
    // practice_16_2();
    practice_16_3();
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

fn practice_16_2() {
    // {
    //     // mpscはmultiple producer, single consumerを表す
    //     // txは転送機、rxは受信機
    //     let (tx, rx) = mpsc::channel();
    // 
    //     thread::spawn(move || {
    //         let val = String::from("hi");
    //         tx.send(val).unwrap();
    //         // valがmoveされて所有権が移っているので使えない
    //         // println!("val is {}", val);
    //     });
    // 
    //     // recvはメインスレッドの実行をブロックし、値がチャンネルを流れてくるまで待機する
    //     // try_recvはブロックせずに即座に返す
    //     let received = rx.recv().unwrap();
    //     println!("Got: {}", received);
    // }

    // 複数の値を送信し、受信側が待機するのを確かめる
    // {
    //     let (tx, rx) = mpsc::channel();
    // 
    //     thread::spawn(move || {
    //         let vals = vec![
    //             String::from("hi"),
    //             String::from("from"),
    //             String::from("the"),
    //             String::from("thread"),
    //         ];
    // 
    //         for val in vals {
    //             tx.send(val).unwrap();
    //             thread::sleep(Duration::from_secs(1));
    //         }
    //     });
    // 
    //     for received in rx {
    //         println!("Got: {}", received);
    //     }
    // }
    
    // 転送機をクローンして複数の生成器を作成する
    {
        let (tx, rx) = mpsc::channel();
        
        let tx1 = mpsc::Sender::clone(&tx);
        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];
            
            for val in vals {
                tx1.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });
        
        thread::spawn(move || {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });
        
        for received in rx {
            println!("Got: {}", received);
        }
    }
}

fn practice_16_3() {
    // Mutex<T>のAPI
    {
        let m = Mutex::new(5);
        {
            // ロックを取得する
            let mut num = m.lock().unwrap();
            *num = 6;
        }
        
        println!("m = {:?}", m);
    }
    
    // 複数のスレッド間でMutex<T>を共有する
    // 複数のスレッドで複数の所有権
    // Arc<T>で原子的な参照カウント
    {
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];
        
        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap();
                *num += 1;
            });
            handles.push(handle);
        }
        
        for handle in handles {
            handle.join().unwrap();
        }
        
        println!("Result: {}", *counter.lock().unwrap());
    }
}
