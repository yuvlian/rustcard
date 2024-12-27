#![allow(dead_code)]
mod asset_n_cfg;
mod card_gen;
mod utils;

use card_gen::create_card;

use mihomo4::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let lang = Language::En;

    let sr = Mihomo::fetch_user(802775147, &lang, &client).await?;
    let ch = sr.clone().get_character_by_name("Firefly").unwrap().clone();
    let ch2 = sr.clone().get_character_by_name("Fu Xuan").unwrap().clone();
    let plr = sr.player.clone();

    let test = create_card(&ch, &plr, None).await?;
    test.save("fftest.png")?;
    let test = create_card(&ch2, &plr, None).await?;
    test.save("fxtest.png")?;

    Ok(())
}
