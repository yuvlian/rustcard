mod card;
mod config;
mod util;

use card::*;
use config::*;
use mihomo4::*;
use util::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dark_block = create_dark_block(100, 100);
    dark_block.save("dark_block.png")?;

    let light_block = create_light_block(100, 100);
    light_block.save("light_block.png")?;

    let rounded_mask = create_rounded_mask((200, 200), 50);
    rounded_mask.save("rounded_mask.png")?;

    let gradient_img = linear_gradient_img((200, 100), (255, 0, 0), (0, 0, 255));
    gradient_img.save("gradient_img.png")?;

    let relic_bg = create_relic_background((300, 150), 5);
    relic_bg.save("relic_bg.png")?;

    let client = Client::new();
    let lang = Language::En;

    let sr = Mihomo::fetch_user(802775147, &lang, &client).await?;
    let ch = sr.clone().get_character_by_name("Firefly").unwrap().clone();
    let plr = sr.player.clone();

    let test = create_card(&ch, &plr, None).await?;
    test.save("fftest.png")?;

    Ok(())
}
