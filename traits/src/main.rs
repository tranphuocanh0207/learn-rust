use aggregator::{Summary, NewsArticle,Tweet, notify, notify_1, notify_2, Pair};

fn main() {
    let sum = NewsArticle {
        headline: String::from("Headline"),
        location: String::from("Location"),
        content: String::from("Content"),
        author: String::from("Author"),
    };
    println!("Summarize: {}", sum.summarize());
    println!("{}", sum.recall_summarize());

    let tweet = Tweet { 
        username: String::from("User Name"),
        content: String::from("Content"),
        reply: true,
        retweet: true
    };

    println!("Tweet: {}", tweet.summarize());

    notify(&sum);
    notify(&tweet);
    
    //trait bound syntax
    let sum2 = NewsArticle {
        headline: "Head line2".to_string(),
        location: "Location2".to_string(),
        author: "Author".to_string(),
        content: "Content".to_string(),
    };
    notify_1(&sum, &tweet);
    notify_2(&sum, &sum2);

    //Pair
    let pair = Pair::new(1, 2);
    println!("Pair: {:?}", pair);
}
