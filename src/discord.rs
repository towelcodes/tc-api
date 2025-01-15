use std::sync::Arc;
use serenity::{async_trait, Client};
use serenity::all::Presence;
use serenity::prelude::*;

struct SharedState {
    presence: Presence
}
struct Handler {
    state: Arc<Mutex<SharedState>>
}
#[async_trait]
impl EventHandler for Handler {
    async fn presence_update(&self, ctx: Context, new_data: Presence) {
        new_data.user.id.to_string();
    }
}

pub async fn init(target: &str) {
    let token = std::env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN must be set to enable Discord integration");
    let intents = GatewayIntents::GUILD_PRESENCES | GatewayIntents::GUILD_MEMBERS | GatewayIntents::GUILD_MESSAGES;
    // let mut client = Client::builder(&token, intents).event_han
}