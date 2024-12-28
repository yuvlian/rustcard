#![allow(dead_code)]
mod asset_n_cfg;
mod card_gen;
mod utils;

use asset_n_cfg::consts::CARD_OUTPUT_PATH;
use card_gen::create_card;

use mihomo4::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let lang = Language::En;

    let sr = Mihomo::fetch_user(803871389, &lang, &client).await?;
    let character_name = "Sparkle";

    let test = create_card(
        &client,
        sr.clone().get_character_by_name(character_name).unwrap(),
        sr.player,
        None,
    )
    .await?;

    test.save(format!("{}{}.png", CARD_OUTPUT_PATH, character_name))?;

    Ok(())
}
