use std::fmt::{format, Debug, Display};

fn main() {
    // practice_10_1()
    // practice_10_2()
    practice_10_3()
}

fn practice_10_1() {
    // 構造体定義
    {
        let point = Point { x: 5, y: 10 };
        let point = Point { x: 1.0, y: 4.0 };
    }
    
    // enum定義
    // OptionやResult型など
    
    // メソッド定義
    {
        let point = Point { x: 5, y: 10 };
        println!("x: {}", point.x());
    }
}

struct Point<T> {
    x: T,
    y: T,
}

impl <T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn practice_10_2() {
    {
        let tweet = Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                // もちろん、ご存知かもしれませんがね、みなさん
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        };

        println!("1 new tweet: {}", tweet.summarize());

        let article = NewsArticle {
            // ペンギンチームがスタンレーカップチャンピオンシップを勝ち取る！
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            // アメリカ、ペンシルベニア州、ピッツバーグ
            location: String::from("Pittsburgh, PA, USA"),
            // アイスバーグ
            author: String::from("Iceburgh"),
            // ピッツバーグ・ペンギンが再度NHL(National Hockey League)で最強のホッケーチームになった
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
            ),
        };

        println!("New article available! {}", article.summarize());
        
        // 引数としてのトレイト
        notify(&article);
    }
}

pub trait Summary {
    fn summarize_author(&self) -> String;

    // デフォルト実装
    fn summarize(&self) -> String {
        // "（{}さんの文章をもっと読む）"
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }

    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// 引数にトレイトを指定
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// 複数の引数がジェネリクスを利用する場合はこちらの書き方のほうがスッキリする
pub fn notify_complex<T: Summary>(item1: &T, item2: &T) {
}

// where句を使ったより明確なトレイト境界
fn some_function<T, U>(t: &T, u: &U) -> i32 where T: Display + Clone, U: Clone + Debug
{
    0
}

struct Pair<T> {
    x: T,
    y: T,
}

impl <T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// 大小比較するためにPartialOrd、出力するためにDisplayを実装していないといけない
impl <T: PartialOrd + Display> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn practice_10_3() {
    // 関数シグニチャにおけるライフタイム注釈
    {
        let string1 = String::from("abcd");
        let string2 = "xyz";

        let result = longest(string1.as_str(), string2);
        // 最長の文字列は、{}です
        println!("The longest string is {}", result);
    }

    // ライフタイムが異なる例
    {
        let string1 = String::from("long string is long");
        {
            let string2 = String::from("xyz");
            // resultがstring1とstring2の内、狭いスコープ内で利用されているので問題なし
            let result = longest(string1.as_str(), string2.as_str());
            println!("The longest string is 「{}」", result);
        }
    }

    // ライフタイムが異なる例
    {
        let string1 = String::from("long string is long");
        // resultがstring1とstring2の内、広いスコープ内で利用されているので問題あり
        let result;
        {
            let string2 = String::from("xyz");
            result = longest(string1.as_str(), string2.as_str());
        }
        // println!("The longest string is {}", result);
    }
    
    // 構造体定義のライフタイム注釈
    {
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");
        let i = ImportantExcerpt { part: first_sentence };
        println!("First sentence: {}", i.part);
    }
}

// x、ｙのどちらの参照を返すのかがわからない
// → ライフタイムの注記が必要
// 引数xとyの両方と戻り値に同じ記号でマークすることで、どちらの引数を返しても生存期間は同じであることを表現できる
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// ImportantExcerptのインスタンスがpartフィールドに保持している参照よりも長生きしない
struct ImportantExcerpt<'a> {
    part: &'a str,
}