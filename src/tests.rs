use super::{comic_url, date};
use chrono::{Duration, NaiveDate};

#[tokio::test]
async fn first_date() {
    let date = date::first();
    assert_eq!(date.to_string(), "1978-06-19");

    let image = comic_url(date).await.expect("Failed to get image");
    assert_eq!(
        image,
        "https://assets.amuniversal.com/aead3a905f69012ee3c100163e41dd5b"
    );
}

#[tokio::test]
async fn recent_dates() {
    let mut date = date::first();

    comic_url(date).await.expect("Failed to get image");

    for _ in 0..2 {
        date -= Duration::days(1);
        comic_url(date).await.expect("Failed to get image");
    }
}

#[tokio::test]
async fn random_dates() {
    for _ in 0..2 {
        let date = date::random();
        comic_url(date).await.expect("Failed to get image");
    }
}

#[tokio::test]
async fn specific_dates() {
    let date = NaiveDate::from_ymd_opt(1978, 11, 4).expect("Date should parse");
    let image = comic_url(date).await.expect("Failed to get image");
    assert_eq!(
        image,
        "https://assets.amuniversal.com/a532a4c05f48012ee3c100163e41dd5b"
    );

    let date = NaiveDate::from_ymd_opt(2020, 4, 26).expect("Date should parse");
    let image = comic_url(date).await.expect("Failed to get image");
    assert_eq!(
        image,
        "https://assets.amuniversal.com/65678e403be70138ebd0005056a9545d"
    );

    let date = NaiveDate::from_ymd_opt(1995, 2, 4).expect("Date should parse");
    let image = comic_url(date).await.expect("Failed to get image");
    assert_eq!(
        image,
        "https://assets.amuniversal.com/425c94c09a8c01365660005056a9545d"
    );
}
