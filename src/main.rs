mod commands;

use std::env;

use rand::rngs::ThreadRng;
use serenity::async_trait;
use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::model::application::{Command, Interaction};
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::*;

use serde::Deserialize;

#[derive(Clone, Deserialize)]
struct Config {
    guild_id: u64,
    discord_token: String,
}

struct Handler {
    // rng: ThreadRng,
    guild_id: u64,
}

impl From<Config> for Handler {
    fn from(value: Config) -> Self {
        Self {
            // rng: rand::thread_rng(),
            guild_id: value.guild_id,
        }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            println!("Received command interaction: {command:#?}");

            let content = match command.data.name.as_str() {
                "ping" => Some(commands::ping::run(&command.data.options())),
                "secret_santa" => {
                    Some(commands::secret_santa::run(&ctx, &command.data.options()).await)
                }
                // "id" => Some(commands::id::run(&command.data.options())),
                // "attachmentinput" => Some(commands::attachmentinput::run(&command.data.options())),
                // "modal" => {
                //     commands::modal::run(&ctx, &command).await.unwrap();
                //     None
                // }
                _ => None,
            };

            if let Some(content) = content {
                let data = CreateInteractionResponseMessage::new().content(content);
                let builder = CreateInteractionResponse::Message(data);
                if let Err(why) = command.create_response(&ctx.http, builder).await {
                    println!("Cannot respond to slash command: {why}");
                }
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let guild_id = GuildId::new(self.guild_id);

        let commands = guild_id
            .set_commands(
                &ctx.http,
                vec![
                    commands::ping::register(),
                    commands::secret_santa::register(),
                    // commands::id::register(),
                    // commands::welcome::register(),
                    // commands::numberinput::register(),
                    // commands::attachmentinput::register(),
                    // commands::modal::register(),
                ],
            )
            .await;

        println!("I now have the following guild slash commands: {commands:#?}");

        // let guild_command =
        //     Command::create_global_command(&ctx.http, commands::wonderful_command::register())
        //         .await;

        // println!("I created the following global slash command: {guild_command:#?}");
    }
}

#[tokio::main]
async fn main() {
    // Get the config
    let file = std::fs::read_to_string("./config.toml")
        .expect("Could not read ./config.toml, make sure the file exists and is readable");
    let config: Config = toml::from_str(&file).unwrap();
    // Configure the client with your Discord bot token in the environment.
    let token = config.discord_token.clone();
    let intents = GatewayIntents::DIRECT_MESSAGES;

    // Build our client.
    let mut client = Client::builder(token, intents)
        .event_handler(Handler::from(config.clone()))
        .await
        .expect("Error creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform exponential backoff until
    // it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
