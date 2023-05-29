use crate::storage::{Entry, Store};
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use shuttle_persist::PersistInstance;
// use tracing::{error, info};

pub struct Bot;

pub struct State {
    pub persist: PersistInstance,
}

impl TypeMapKey for State {
    type Value = State;
}

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, msg: Message) {
        let mut data = ctx.data.write().await;
        let mut store = data
            .get_mut::<State>()
            .expect("")
            .persist
            .load::<Store>(crate::PERSISTENT_STORE_KEY)
            .unwrap_or_default();

        if msg.content == "!list" {
            if let Err(e) = msg
                .channel_id
                .say(
                    &ctx.http,
                    format!(
                        "Following:\n{}",
                        store
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
                // error!("Error sending message: {:?}", e);
            }
        }

        if msg.content.starts_with("!follow") {
            let username = msg.content[8..].trim();

            if store
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
                    // error!("Error sending message: {:?}", e);
                }
            } else {
                store.entries.push(Entry {
                    username: username.to_string(),
                    latest_track_uri: "".to_string(),
                    latest_track_title: "".to_string(),
                });

                if let Err(e) = msg
                    .channel_id
                    .say(&ctx.http, format!("Now following: [{}]", username))
                    .await
                {
                    // error!("Error sending message: {:?}", e);
                }
            }
        }

        if msg.content.starts_with("!unfollow") {
            let username = msg.content[10..].trim();
            store.entries.retain(|e| e.username != username);

            if let Err(e) = msg
                .channel_id
                .say(&ctx.http, format!("Stopped following: {}", username))
                .await
            {
                // error!("Error sending message: {:?}", e);
            }
        }

        data.get_mut::<State>()
            .expect("")
            .persist
            .save::<Store>(crate::PERSISTENT_STORE_KEY, store)
            .unwrap();
    }

    async fn ready(&self, _: Context, ready: Ready) {
        // info!("{} is connected!", ready.user.name);
    }
}
