/*
 * @Author: Lmmqxyx
 * @Date: 2022-07-03 23:56:11
 * @LastEditors: 
 * @LastEditTime: 2022-07-03 23:58:50
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

// trait can bind function
#[test]
fn middle_3_trait() {
    let news=NewsArticle{
        headline:String::from("ceshiyixia"),
        location:String::from("hefei"),
        author:String::from("si"),
        content:String::from("zhaiyaoshifen"),
    };
    println!("{}",news.summarize());
}
