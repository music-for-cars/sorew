use anyhow::anyhow;
use serenity::prelude::*;
use shuttle_persist::PersistInstance;
use shuttle_secrets::SecretStore;

mod bot;
mod error;
mod scscraper;
mod storage;
mod watcher;

const MFC_RELEASES_CHANNEL_ID: u64 = 1111056725504704592;
const PERSISTENT_STORE_KEY: &'static str = "store";
const WATCHER_INTERVAL_SECONDS: u64 = 300;

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
    #[shuttle_persist::Persist] persist: PersistInstance,
) -> Result<shuttle_serenity::SerenityService, shuttle_runtime::Error> {
    // Get the discord token set in `Secrets.toml`
    let token = if let Some(token) = secret_store.get("DISCORD_TOKEN") {
        token
    } else {
        return Err(anyhow!("'DISCORD_TOKEN' was not found").into());
    };

    let state = bot::State {
        persist: persist.clone(),
    };

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let client = Client::builder(&token, intents)
        .event_handler(bot::Bot)
        .type_map_insert::<bot::State>(state)
        .await
        .expect("Err creating client");

    // Setup the watcher loop
    tokio::spawn(watcher::create(token, persist, WATCHER_INTERVAL_SECONDS));

    // Return the discord bot client, shuttle will run it for us
    Ok(client.into())
}
