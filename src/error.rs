use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Data load error: {0}")]
    LoadDataError(String),

    #[error("Data deserialzie error: {0}")]
    DeserializeDataError(String),

    #[error("Data save error: {0}")]
    SaveDataError(String),

    #[error("Data serialize error: {0}")]
    SerializeDataError(String),

    #[error("Scraper fetch error: {0}")]
    ScrapeFetchError(String),

    #[error("Scraper parse error: {0}")]
    ScrapeParseError(String),
}
