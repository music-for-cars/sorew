use crate::{scscraper, storage::Store};
use serenity::model::prelude::ChannelId;
use shuttle_persist::PersistInstance;
use std::future::Future;
// use tracing::error;

pub fn create(
    token: String,
    persist: PersistInstance,
    interval_seconds: u64,
) -> impl Future<Output = ()> {
    async move {
        let http = serenity::http::Http::new(&token);
        loop {
            let mut store = persist
                .load::<Store>(crate::PERSISTENT_STORE_KEY)
                .unwrap_or_default();

            for stored_entry in store.entries.iter_mut() {
                if let Ok(entry) =
                    scscraper::get_latest_uri_and_title_for_username(&stored_entry.username).await
                {
                    if entry.latest_track_uri != stored_entry.latest_track_uri {
                        send_release_message(
                            &http,
                            crate::MFC_RELEASES_CHANNEL_ID,
                            &entry.latest_track_uri,
                        )
                        .await;
                        stored_entry.latest_track_uri = entry.latest_track_uri;
                        stored_entry.latest_track_title = entry.latest_track_title;
                    }
                } else {
                    // error!("Error processing for '{}'", stored_entry.username);
                }
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            }

            // After all entries are processed, update the persistent storage
            persist.save("store", store).unwrap();

            tokio::time::sleep(tokio::time::Duration::from_secs(interval_seconds)).await;
        }
    }
}

async fn send_release_message(http: &serenity::http::Http, channel_id: u64, track_uri: &str) {
    ChannelId(channel_id).say(http, track_uri).await.unwrap();
}
