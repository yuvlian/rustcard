use crate::asset_n_cfg::font::get_font;
use crate::utils::draw_blocks::draw_dark_block;
use image::{DynamicImage, RgbaImage};
use mihomo4::{CharacterData, Client, PlayerData};
use std::error::Error;

mod char_img;
mod char_path;
mod char_rank;
mod char_skill;
mod plr_info;

pub async fn create_card(
    cl: &Client,
    ch: &CharacterData,
    plr: PlayerData,
    img_url: Option<&str>,
) -> Result<DynamicImage, Box<dyn Error>> {
    let font = get_font();

    println!("->> [0/8] Rendering: {}", &ch.name);
    let mut img = draw_dark_block(1520, 1339);

    println!("->> [1/8] Character Potrait");
    char_img::render(&mut img, ch, cl, img_url, &font).await?;

    println!("->> [2/8] Character Rank");
    char_rank::render(&mut img, ch, cl).await?;

    println!("->> [3/8] Character Path");
    char_path::render(&mut img, ch, cl).await?;

    println!("->> [4/8] Player Info");
    plr_info::render(&mut img, plr, &font);

    println!("->> [5/8] Character Skill");
    char_skill::render(&mut img, ch, cl, &font).await?;

    println!("->> [6/8] Character Lightcone");
    render_lc(&mut img, ch, &font);

    println!("->> [7/8] Character Stats");
    render_stats(&mut img, ch, &font);

    println!("->> [8/8] Character Relics");
    render_relics(&mut img, ch, &font);

    Ok(DynamicImage::ImageRgba8(img))
}

fn render_lc(_img: &mut RgbaImage, _ch: &CharacterData, _font: &impl ab_glyph::Font) {}
fn render_stats(_img: &mut RgbaImage, _ch: &CharacterData, _font: &impl ab_glyph::Font) {}
fn render_relics(_img: &mut RgbaImage, _ch: &CharacterData, _font: &impl ab_glyph::Font) {}
