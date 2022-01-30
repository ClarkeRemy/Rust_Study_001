use lib::{self, Summary, Tweet};
use lib::NewsArticle;

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championsip!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best \n hockey team in the NHL."),
    };

    println!("1 new tweet: {}", tweet.summarize());

    println!("New article availible! {}", article.summarize());
}
