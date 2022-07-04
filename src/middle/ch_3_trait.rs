/*
 * @Author: Lmmqxyx
 * @Date: 2022-07-03 23:56:11
 * @LastEditors: Please set LastEditors
 * @LastEditTime: 2022-07-04 23:46:48
 * @FilePath: \Rust_Tutorial\src\middle\ch_3_trait.rs
 * @Description:
 */
trait Summary {
    fn summarize(&self) -> String;
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, --by({})", self.headline, self.author)
    }
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn notify_1(item: impl Summary, item2: impl Summary) {
    println!("Breaking news {}", item.summarize());
}

fn notify_2<T: Summary>(item: T, item2: T) {
    println!("Breaking news notify2 {}", item.summarize());
}
// trait can bind function
#[test]
fn middle_3_trait() {
    let news = NewsArticle {
        headline: String::from("ceshiyixia"),
        location: String::from("hefei"),
        author: String::from("si"),
        content: String::from("zhaiyaoshifen"),
    };
    println!("{}", news.summarize());
}
