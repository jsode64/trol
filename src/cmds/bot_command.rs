use crate::cmds::{conch::CONCH_CMD, poke::POKE_CMD};

use serenity::all::{
    CommandInteraction, Context, CreateCommand, CreateInteractionResponseFollowup,
};

/// A Discord bot command.
#[derive(Clone, Copy, Debug)]
pub struct BotCommand {
    /// The command's name.
    pub name: &'static str,

    /// The command's description.
    pub description: &'static str,

    /// The commannd's response function.
    pub respond: fn() -> CreateInteractionResponseFollowup,
}

impl BotCommand {
    /// Returns a `CreateCommand` for the command.
    pub fn create_command(&self) -> CreateCommand {
        CreateCommand::new(self.name).description(self.description)
    }

    /// Runs the command on the given context.
    pub async fn run(&self, ctx: &Context, interaction: &CommandInteraction) {
        if let Err(e) = interaction.defer(&ctx.http).await {
            println!("{e}");
        }

        let response = (self.respond)();

        if let Err(e) = interaction.create_followup(&ctx.http, response).await {
            println!("{e}");
        }
    }
}

/// An array of all commands.
pub static COMMANDS: [BotCommand; 2] = [POKE_CMD, CONCH_CMD];
