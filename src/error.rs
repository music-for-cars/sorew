use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Scraper fetch error: {0}")]
    ScrapeFetchError(String),

    #[error("Scraper parse error: {0}")]
    ScrapeParseError(String),
}
