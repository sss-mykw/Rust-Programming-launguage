fn main() {
    practice_5_1();
    
}

fn practice_5_1() {
    let user1 = build_user(String::from("<EMAIL>"), String::from("someusername123"));
    
    // mutを付けるとフィールドが可変になる
    let mut user2 = build_user(String::from("<EMAIL>"), String::from("someusername123"));
    user2.email = String::from("<EMAIL>hoge");
    
    // 構造体更新記法
    let user3 = User {
        email: String::from("<EMAIL>"),
        username: String::from("someusername123"),
        ..user1
    };
}

fn build_user(email: String, username: String) -> User {
    // フィールド初期化省略記法
    // メソッドの引数とフィールド名が同一の場合に使用可能
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}