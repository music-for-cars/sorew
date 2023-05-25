use crate::{error::Error, storage::Entry};

pub async fn get_latest_uri_and_title_for_username<S>(username: S) -> Result<Entry, Error>
where
    S: Into<String>,
{
    let username = username.into();

    let content = reqwest::get(format!("https://soundcloud.com/{}/tracks", username))
        .await
        .map_err(|e| Error::ScrapeFetchError(e.to_string()))?
        .text()
        .await
        .map_err(|e| Error::ScrapeFetchError(e.to_string()))?;
    let document = scraper::Html::parse_document(&content);

    // The data we need is in a <noscript> tag, somehow scraper doesn't directly parse the html in
    // there so we need a little trickery here to re-evaluate that inner part
    let selector =
        scraper::Selector::parse("noscript").map_err(|e| Error::ScrapeParseError(e.to_string()))?;
    let inner_content = document
        .select(&selector)
        .last()
        .ok_or_else(|| Error::ScrapeParseError("noscript tag not found".to_string()))?
        .inner_html();
    let inner_content_decoded = html_escape::decode_html_entities(&inner_content);
    let inner_document = scraper::Html::parse_document(&inner_content_decoded);

    // Now we can look for the first h2 a[itemprop] which currently matches the latest track
    let inner_selector = scraper::Selector::parse("h2 a[itemprop]")
        .map_err(|e| Error::ScrapeParseError(e.to_string()))?;
    let element = inner_document
        .select(&inner_selector)
        .next()
        .ok_or_else(|| Error::ScrapeParseError("first track tag not found".to_string()))?;

    let latest_track_uri = format!(
        "https://soundcloud.com{}",
        element
            .value()
            .attr("href")
            .ok_or_else(|| Error::ScrapeParseError("noscript tag not found".to_string()))?
    );
    let latest_track_title = element.inner_html();

    Ok(Entry {
        username,
        latest_track_uri,
        latest_track_title,
    })
}

#[test]
fn test_scraper() {
    let content = std::fs::read_to_string("test/tracks.html").unwrap();
    let document = scraper::Html::parse_document(&content);
    let selector = scraper::Selector::parse("noscript").unwrap();
    let inner_content = document.select(&selector).last().unwrap().inner_html();
    let inner_content_decoded = html_escape::decode_html_entities(&inner_content);
    let inner_document = scraper::Html::parse_document(&inner_content_decoded);
    let inner_selector = scraper::Selector::parse("h2 a[itemprop]").unwrap();

    let element = inner_document.select(&inner_selector).next().unwrap();

    let uri = element.value().attr("href").unwrap();
    let title = element.inner_html();

    assert_eq!(uri, "/derk-bell/epilogue".to_string());
    assert_eq!(title, "Epilogue");
}
