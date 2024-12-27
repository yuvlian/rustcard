use crate::asset_n_cfg::font::get_font;
use crate::utils::draw_blocks::draw_dark_block;
use image::{DynamicImage, RgbaImage};
use mihomo4::{CharacterData, PlayerData};
use std::error::Error;

mod char_img;
mod char_rank;
mod char_path;

pub async fn create_card(
    ch: &CharacterData,
    plr: &PlayerData,
    img_url: Option<&str>,
) -> Result<DynamicImage, Box<dyn Error>> {
    let mut img = draw_dark_block(1520, 1339);
    let font = get_font();

    println!("->> Rendering: {}", &ch.name);
    println!("->> Character Potrait");
    char_img::render(&mut img, ch, img_url, &font).await?;

    println!("->> Character Rank");
    char_rank::render(&mut img, ch).await?;

    render_skill(&mut img, ch, &font);

    println!("->> Character Path");
    char_path::render(&mut img, ch).await?;

    render_user(&mut img, plr, &font);
    render_lc(&mut img, ch, &font);
    render_stats(&mut img, ch, &font);
    render_relics(&mut img, ch, &font);

    Ok(DynamicImage::ImageRgba8(img))
}

fn render_skill(_img: &mut RgbaImage, _ch: &CharacterData, _font: &impl ab_glyph::Font) {}
fn render_path(_img: &mut RgbaImage, _ch: &CharacterData, _font: &impl ab_glyph::Font) {}
fn render_user(_img: &mut RgbaImage, _plr: &PlayerData, _font: &impl ab_glyph::Font) {}
fn render_lc(_img: &mut RgbaImage, _ch: &CharacterData, _font: &impl ab_glyph::Font) {}
fn render_stats(_img: &mut RgbaImage, _ch: &CharacterData, _font: &impl ab_glyph::Font) {}
fn render_relics(_img: &mut RgbaImage, _ch: &CharacterData, _font: &impl ab_glyph::Font) {}
