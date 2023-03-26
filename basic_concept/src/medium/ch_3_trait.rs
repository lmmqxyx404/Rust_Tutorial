/*
 * @Author: Lmmqxyx
 * @Date: 2022-07-03 23:56:11
 * @LastEditors: Please set LastEditors
 * @LastEditTime: 2022-07-06 06:44:20
 * @FilePath: \Rust_Tutorial\src\middle\ch_3_trait.rs
 * @Description:
 */

/// In my opinion trait is used for define operations by add different function.
/// And a function set can be considered to be a trait.
/// so a trait can be inherited from another trait.
/// also trait can used for function arguments and returned value.
/// pay attention to the case that a trait bounds the returned value.
use std::fmt::{Debug, Display};
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
fn notify_3<T, U>(item: T, item2: U, item3: i32) -> Box<dyn Summary>
where
    T: Summary + Display,
    U: Clone + Debug,
{
    // println!("Breaking news notify2 {}", item.summarize());
    if item3 > 2 {
        Box::new(Tweet {
            username: String::from("ceshiyixia"),
            content: String::from("hefei"),
            reply: false,
            retweet: false,
        })
    } else {
        Box::new(NewsArticle {
            headline: String::from("ceshiyixia"),
            location: String::from("hefei"),
            author: String::from("si"),
            content: String::from("zhaiyaoshifen"),
        })
    }
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
