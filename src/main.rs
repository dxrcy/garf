use chrono::Duration;
use garf::{comic_url, date};

#[tokio::main]
async fn main() {
    let today = date::today();

    let image = comic_url(today).await.expect("Failed");
    println!("{}", image);

    let yesterday = today - Duration::days(1);

    let image = comic_url(yesterday).await.expect("Failed");
    println!("{}", image);

    let image = comic_url(date::first()).await.expect("Failed");
    println!("{}", image);

    let image = comic_url(date::first() - Duration::days(1))
        .await
        .expect("Failed");
    println!("{}", image);

    for _ in 0..10 {
        let date = date::random();
        println!("{}", date);
        let image = comic_url(date).await.expect("Failed");
        println!("{}", image);
    }
}
