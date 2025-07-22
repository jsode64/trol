use crate::cmds::bot_command::{BotCommand, COMMANDS};

use serenity::{
    all::{
        ClientBuilder, Command, Context, EventHandler, GatewayIntents, Interaction, Message, Ready,
    },
    async_trait,
};

mod cmds;

struct Bot;

#[async_trait]
impl EventHandler for Bot {
    async fn ready(&self, ctx: Context, ready: Ready) {
        // Successful launch message.
        println!("{} is up", ready.user.name);

        // Print connected guilds (servers).
        for guild in ready.guilds {
            println!("Connected to {}", guild.id);
        }

        // Set commands.
        Command::set_global_commands(
            &ctx.http,
            COMMANDS.iter().map(BotCommand::create_command).collect(),
        )
        .await
        .unwrap();
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
        // Give a thumbs down whenever `tuftydragon943` sends a message.
        if msg.author.name == "tuftydragon943" {
            let _ = msg.react(&ctx.http, '👎').await;
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
        .expect("Ok");

    if let Err(e) = client.start().await {
        println!("{e}");
    }

    Ok(())
}
