use crate::cmds::bot_command::BotCommand;

use rand::{rng, Rng};
use serenity::all::{CreateEmbed, CreateInteractionResponseFollowup};

/// The number of Pokémon. If too large, will lead to invalid URLs.
const NUM_POKEMON: usize = 1025;

/// The URL to the `pokemon.com` image sources without the index number and `.png` tag.
const IMG_URL: &str = "https://www.pokemon.com/static-assets/content-assets/cms2/img/pokedex/full/";

/// `/poke` command response function.
fn run() -> CreateInteractionResponseFollowup {
    CreateInteractionResponseFollowup::new().embed(CreateEmbed::new().image(format!(
        "{IMG_URL}{:03}.png",
        rng().random_range(0..=NUM_POKEMON),
    )))
}

pub static POKE_CMD: BotCommand = BotCommand {
    name: "poke",
    description: "Get an image of a random Pokémon",
    respond: run,
};
