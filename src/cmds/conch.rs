use crate::cmds::bot_command::BotCommand;

use once_cell::sync::Lazy;
use rand::{rng, seq::IndexedRandom};
use serenity::all::CreateInteractionResponseFollowup;

/// Potential responses.
const RESPONSES: Lazy<[String; 6]> = Lazy::new(|| {
    [
        "Maybe someday.".to_string(),
        "I don't think so.".to_string(),
        "No.".to_string(),
        "Yes.".to_string(),
        "Try asking again.".to_string(),
        "*No!*".to_string(),
    ]
});

/// `/conch` command response function.
fn run() -> CreateInteractionResponseFollowup {
    CreateInteractionResponseFollowup::new().content(RESPONSES.choose(&mut rng()).unwrap())
}

pub static CONCH_CMD: BotCommand = BotCommand {
    name: "conch",
    description: "We must never question the wisdom of the Magic Conch",
    respond: run,
};
