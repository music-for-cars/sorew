use crate::{scscraper, storage::Store};
use serenity::model::prelude::ChannelId;
use tokio::sync::Mutex;
// use shuttle_persist::PersistInstance;
use std::future::Future;
// use tracing::error;

pub fn create(config: crate::config::Config, state: crate::bot::State) -> impl Future<Output = ()> {
    async move {
        let http = serenity::http::Http::new(&config.discord_token);
        loop {
            // todo: we should use the same arc mutex store
            let store: &Mutex<Store> = state.store.as_ref();

            for stored_entry in store.lock().await.entries.iter_mut() {
                log::warn!("Wil try to parse now");
                match scscraper::get_latest_uri_and_title_for_username(&stored_entry.username).await
                {
                    Ok(entry) => {
                        log::warn!("Parsing was ok, got {}", entry.latest_track_title);
                        if entry.latest_track_uri != stored_entry.latest_track_uri {
                            send_release_message(
                                &http,
                                config.discord_channel_id,
                                &entry.latest_track_uri,
                            )
                            .await;
                            stored_entry.latest_track_uri = entry.latest_track_uri;
                            stored_entry.latest_track_title = entry.latest_track_title;
                        }
                    }
                    Err(error) => {
                        log::error!(
                            "Error processing for '{}': {}",
                            stored_entry.username,
                            error
                        );
                    }
                }

                tokio::time::sleep(tokio::time::Duration::from_secs(
                    config.watcher_delay_between_seconds,
                ))
                .await;
            }

            // After all entries are processed, update the persistent storage
            crate::storage::save_data("data.json", store.lock().await.to_owned()).unwrap();
            tokio::time::sleep(tokio::time::Duration::from_secs(
                config.watcher_interval_seconds,
            ))
            .await;
        }
    }
}

async fn send_release_message(http: &serenity::http::Http, channel_id: u64, track_uri: &str) {
    ChannelId(channel_id).say(http, track_uri).await.unwrap();
}
