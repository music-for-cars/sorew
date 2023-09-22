use std::sync::Arc;

use log::{error, warn};
use storage::{load_data, save_data};
use tokio::sync::Mutex;

mod bot;
mod config;
mod error;
mod scscraper;
mod storage;
mod watcher;

#[tokio::main]
async fn main() {
    simple_logger::init_with_level(log::Level::Info).unwrap();

    use envconfig::Envconfig;
    let config = config::Config::init_from_env().unwrap();

    log::info!("Sorew is starting");
    let store = match load_data::<storage::Store>("data.json") {
        Ok(store) => store,
        Err(e) => {
            warn!("{}", e);
            let store = storage::Store::default();
            save_data("data.json", &store).unwrap();
            store
        }
    };

    let state = bot::State {
        store: Arc::new(Mutex::new(store)),
    };

    // Setup the watcher loop
    tokio::spawn(watcher::create(
        config.clone(),
        bot::State {
            store: Arc::clone(&state.store),
        },
    ));

    if let Err(e) = bot::start(&config.discord_token, state).await.start().await {
        error!("{}", e);
        warn!("Restarting");
    }
}
