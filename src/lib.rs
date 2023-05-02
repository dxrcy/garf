/// Get relevant dates, as `NaiveDate`
pub mod date;
#[cfg(test)]
mod tests;

use chrono::{Datelike, NaiveDate};
use thiserror::Error;

/// Error fetching image URL for comic
#[derive(Debug, Error)]
pub enum ComicError {
    #[error("Request failed")]
    Request(reqwest::Error),
    #[error("Image link was not found in webpage body")]
    ImageLinkNotFound,
    #[error("Image link was not formatted in ASCII text")]
    InvalidImageLinkEncoding,
}

/// Get image URL of comic, asynchronously, given a date (`NaiveDate`)
pub async fn comic_url(date: NaiveDate) -> Result<String, ComicError> {
    // Convert date to YYYY/MM/DD string
    let date_string = date_to_string(date);

    // Get webpage url from date string
    let url = format!(
        "https://corsproxy.garfieldapp.workers.dev/cors-proxy?https://www.gocomics.com/garfield/{}",
        date_string
    );

    // Fetch webpage body
    let response_body = fetch_body(&url).await.map_err(ComicError::Request)?;

    // Find image url in HTML
    let Some(char_index) = response_body.find("https://assets.amuniversal.com") else {
        return Err(ComicError::ImageLinkNotFound);
    };

    // Get string from character index
    let Some(image_url) = response_body.get(char_index..char_index + 63) else {
        return Err(ComicError::InvalidImageLinkEncoding);
    };

    Ok(image_url.to_string())
}

/// Fetch HTML body from URL
async fn fetch_body(url: &str) -> Result<String, reqwest::Error> {
    reqwest::Client::new().get(url).send().await?.text().await
}

/// Convert `NaiveDate` to YYYY/MM/DD format
fn date_to_string(date: NaiveDate) -> String {
    date.year().to_string() + "/" + &date.month().to_string() + "/" + &date.day().to_string()
}
