use chrono::{NaiveDate, NaiveDateTime, NaiveTime, Utc};
use rand::Rng;

/// Today's date, as `NaiveDate`
pub fn today() -> NaiveDate {
    Utc::now().date_naive()
}

/// Date of first comic, as `NaiveDate`
pub fn first() -> NaiveDate {
    NaiveDate::from_ymd_opt(1978, 6, 19).expect("Static date failed to parse")
}

/// Random date, between date of first comic `date::first()`, and today's date
/// (`date::today`)
pub fn random() -> NaiveDate {
    // Get timestamp of first date
    let first_date = first().and_time(NaiveTime::MIN).timestamp();

    // Get timestamp of today's date
    let today_date = Utc::now().timestamp();

    // Convert to range
    let range = first_date..=today_date;

    // Get random timestamp in range
    let timestamp = rand::thread_rng().gen_range(range);

    // Convert timestamp to datetime
    let datetime = NaiveDateTime::from_timestamp_opt(timestamp, 0).unwrap();

    // Convert to date
    datetime.date()
}
