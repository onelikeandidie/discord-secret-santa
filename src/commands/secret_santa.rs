use rand::Rng;
use serenity::all::{
    CommandOptionType, Context, CreateCommandOption, CreateMessage, EventHandler, MessageBuilder,
};
use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

use crate::Handler;

pub async fn run(ctx: &Context, options: &[ResolvedOption<'_>]) -> String {
    let mut users: Vec<&serenity::model::prelude::User> = options
        .iter()
        .filter_map(|option| {
            match option.value {
                // serenity::all::ResolvedValue::Autocomplete { kind, value } => todo!(),
                // serenity::all::ResolvedValue::Boolean(_) => todo!(),
                // serenity::all::ResolvedValue::Integer(_) => todo!(),
                // serenity::all::ResolvedValue::Number(_) => todo!(),
                // serenity::all::ResolvedValue::String(_) => todo!(),
                // serenity::all::ResolvedValue::SubCommand(_) => todo!(),
                // serenity::all::ResolvedValue::SubCommandGroup(_) => todo!(),
                // serenity::all::ResolvedValue::Attachment(_) => todo!(),
                // serenity::all::ResolvedValue::Channel(_) => todo!(),
                // serenity::all::ResolvedValue::Role(_) => todo!(),
                serenity::all::ResolvedValue::User(user, _) => Some(user),
                // serenity::all::ResolvedValue::Unresolved(_) => todo!(),
                _ => None,
            }
        })
        .collect();

    if users.len() < 2 {
        return "More than 1 user is required".to_string();
    }

    // let mut senders = users.clone();
    // let mut receivers = users.clone();
    let mut sender = None;
    let mut first_sender = None;
    while users.len() > 0 {
        let index = {
            let mut rng = rand::thread_rng();
            rng.gen_range(0..users.len())
        };
        let reciever = users.swap_remove(index);
        if sender.is_none() {
            sender = Some(reciever);
            first_sender = Some(reciever);
        } else {
            let message = MessageBuilder::new()
                .push("Your secret santa recipient is: ")
                .mention(reciever)
                .build();
            let message = CreateMessage::new().content(message);
            if let Err(e) = sender.unwrap().dm(&ctx.http, message).await {
                eprintln!("{}", e);
            }
            sender = Some(reciever);
        }
        if users.is_empty() {
            let message = MessageBuilder::new()
                .push("Your secret santa recipient is: ")
                .mention(first_sender.unwrap())
                .build();
            let message = CreateMessage::new().content(message);
            if let Err(e) = reciever.dm(&ctx.http, message).await {
                eprintln!("{}", e);
            }
        }
    }

    "Sent everyone their secret santa recipients!".to_string()
}

pub fn register() -> CreateCommand {
    let mut command = CreateCommand::new("secret_santa")
        .description("Sends a DM to each user with their secret santa!");

    for i in 1..25 {
        let option = if i <= 2 {
            CreateCommandOption::new(
                CommandOptionType::User,
                format!("user_{}", i),
                "One of the secret santas, minimum 2",
            )
        } else {
            CreateCommandOption::new(
                CommandOptionType::User,
                format!("user_{}", i),
                "One of the secret santas",
            )
        };
        command = command.add_option(option);
    }

    command
}
