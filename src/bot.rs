use crate::storage::{self, Entry, Store};
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use std::sync::Arc;
use tokio::sync::Mutex;
// use shuttle_persist::PersistInstance;
// use tracing::{error, info};

pub struct Bot;

pub struct State {
    pub store: Arc<Mutex<Store>>,
}

impl TypeMapKey for State {
    type Value = State;
}

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, msg: Message) {
        let mut data = ctx.data.write().await;
        let store = data.get_mut::<State>().expect("").store.as_ref();

        if msg.content == "!list" {
            if let Err(e) = msg
                .channel_id
                .say(
                    &ctx.http,
                    format!(
                        "Following:\n{}",
                        store
                            .lock()
                            .await
                            .entries
                            .iter()
                            .map(|entry| format!(
                                "- {}: {}\n",
                                entry.username, entry.latest_track_title
                            ))
                            .collect::<String>()
                    ),
                )
                .await
            {
                log::error!("Error sending message: {:?}", e);
            }
        }

        if msg.content.starts_with("!follow") {
            let username = msg.content[8..].trim();

            if store
                .lock()
                .await
                .entries
                .iter()
                .find(|&e| e.username == username)
                .is_some()
            {
                if let Err(e) = msg
                    .channel_id
                    .say(&ctx.http, format!("Already following: {}", username))
                    .await
                {
                    log::error!("Error sending message: {:?}", e);
                }
            } else {
                store.lock().await.entries.push(Entry {
                    username: username.to_string(),
                    latest_track_uri: "".to_string(),
                    latest_track_title: "".to_string(),
                });

                if let Err(e) = msg
                    .channel_id
                    .say(&ctx.http, format!("Now following: [{}]", username))
                    .await
                {
                    log::error!("Error sending message: {:?}", e);
                }
            }
        }

        if msg.content.starts_with("!unfollow") {
            let username = msg.content[10..].trim();
            store
                .lock()
                .await
                .entries
                .retain(|e| e.username != username);

            if let Err(e) = msg
                .channel_id
                .say(&ctx.http, format!("Stopped following: {}", username))
                .await
            {
                log::error!("Error sending message: {:?}", e);
            }
        }

        storage::save_data("data.json", store.lock().await.to_owned()).unwrap();
    }

    async fn ready(&self, _: Context, ready: Ready) {
        log::info!("{} is connected!", ready.user.name);
    }
}

pub async fn start(token: &str, state: State) -> Client {
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    Client::builder(&token, intents)
        .event_handler(Bot)
        .type_map_insert::<State>(state)
        .await
        .expect("Err creating client")
}
