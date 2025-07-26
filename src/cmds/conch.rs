use crate::cmds::bot_command::BotCommand;

use rand::{rng, seq::IndexedRandom};
use serenity::all::CreateInteractionResponseFollowup;

/// Potential responses.
const RESPONSES: [&str; 6] = [
    "Maybe someday.",
    "I don't think so.",
    "No.",
    "Yes.",
    "Try asking again.",
    "*No!*",
];

/// `/conch` command response function.
fn run() -> CreateInteractionResponseFollowup {
    CreateInteractionResponseFollowup::new().content(*RESPONSES.choose(&mut rng()).unwrap())
}

pub static CONCH_CMD: BotCommand = BotCommand {
    name: "conch",
    description: "We must never question the wisdom of the Magic Conch",
    respond: run,
};
