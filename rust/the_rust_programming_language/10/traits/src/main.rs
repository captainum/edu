mod aggregator {
    // pub trait Summary {
    //     fn summarize(&self) -> String;
    // }

    // default behaviour
    pub trait Summary {
        fn summarize(&self) -> String {
            format!("(Read more from {}...)", self.summarize_author())
        }

        fn summarize_author(&self) -> String;
    }

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }

        fn summarize_author(&self) -> String {
            panic!("not implemented!")
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        // fn summarize(&self) -> String {
        //     format!("{}: {}", self.username, self.content)
        // }

        fn summarize_author(&self) -> String {
            format!("@{}", self.username)
        }
    }

    // pub fn notify(item: &impl Summary) {
    //     println!("Breaking news! {}", item.summarize());
    // }

    // pub fn notify<T: Summary>(item: &T) {}

    // pub fn notify(item1: &impl Summary, item2: &impl Summary) {}

    // pub fn notify<T: Summary>(item1: &T, item2: &T) {}

    // pub fn notify(item: &(impl Summary + Display)) {}

    // pub fn notify<T: Summary + Display>(item: &T) {}

    // pub some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}

    // pub fn some_function<T, U>(t: &T, u: &U) -> i32
    // where
    //     T: Display + Clone,
    //     U: Clone + Debug,
    // {}

    // pub fn return_summarizable() -> impl Summary {}
}

use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {x, y}
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn main() {
    use aggregator::{Tweet, Summary};

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}
