use crate::cmds::bot_command::{BotCommand, COMMANDS};

use rand::{rng, seq::IndexedRandom};
use serenity::{
    all::{
        ClientBuilder, Command, Context, EventHandler, GatewayIntents, Interaction, Message, Ready,
    },
    async_trait,
};

mod cmds;
mod util;

struct Bot;

#[async_trait]
impl EventHandler for Bot {
    async fn ready(&self, ctx: Context, ready: Ready) {
        // Successful launch message.
        log_cmd!("{} went online", ready.user.name);

        // Set commands.
        log_err!(
            Command::set_global_commands(
                &ctx.http,
                COMMANDS.iter().map(BotCommand::create_command).collect(),
            )
            .await
        );
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(cmd) = interaction {
            // Find which command was called and run it.
            for command in COMMANDS {
                if command.name == cmd.data.name {
                    command.run(&ctx, &cmd).await;
                    break;
                }
            }
        }
    }

    async fn message(&self, ctx: Context, msg: Message) {
        // Give a negative reaction whenever `tuftydragon943` sends a message.
        if msg.author.name == "tuftydragon943" {
            let &c = ['👎', '🙁', '❌', '💔'].choose(&mut rng()).unwrap();

            log_err!(msg.react(&ctx.http, c).await);
            log_cmd!("Reacted '{}' to Palma: \"{}\"", c, msg.content);
        }

        // Politely tell the author not to ping everyone.
        if msg.mention_everyone {
            log_err!(msg.reply_mention(&ctx.http, "***KILL YOURSELF!***").await);
            log_cmd!("Told `{}` not to ping everyone", msg.author.name);
        }

        // Decypher the message, using its words' first letters as hints to a BTD6 co-op code.
        if msg.mentions_user_id(ctx.cache.current_user().id) {
            // Get a string via the first word hints.
            let letters = msg
                .content
                .split_ascii_whitespace()
                .skip(1)
                .map(|x| x.chars().nth(0).unwrap().to_ascii_uppercase())
                .collect::<String>();

            // Verify the string.
            if letters.len() == 6 && letters.chars().all(|c| c.is_ascii_alphabetic()) {
                log_err!(
                    msg.channel_id
                        .say(&ctx.http, format!("THE CODE IS **{letters}** !!!"))
                        .await
                );
                log_cmd!(
                    "Got code \"{}\" from `{}`'s message: \"{}\"",
                    letters,
                    msg.author.name,
                    &msg.content[22..].trim_start()
                );
            } else {
                log_err!(msg.react(&ctx.http, '🥀').await);
                log_cmd!(
                    "Failed to get a code from `{}`'s message: \"{}\"",
                    msg.author.name,
                    &msg.content[22..].trim_start()
                )
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), String> {
    use dotenv::var;

    let token = var("TOKEN").unwrap();
    let intents = GatewayIntents::GUILD_MESSAGES;
    let mut client = ClientBuilder::new(&token, intents)
        .event_handler(Bot)
        .await
        .map_err(|e| e.to_string())?;

    client.start().await.map_err(|e| e.to_string())?;

    Ok(())
}
