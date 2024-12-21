use crate::util::*;
use image::buffer::ConvertBuffer;
use image::*;
use imageproc::*;
use mihomo4::*;
use rusttype::*;
use std::error::Error;

pub fn print() {
    println!("hello");
}

pub async fn create_card(
    ch: &CharacterData,
    plr: &PlayerData,
    img_url: Option<&str>,
) -> Result<DynamicImage, Box<dyn std::error::Error>> {
    let mut img = create_dark_block(1520, 1339);

    render_char_img(&mut img, ch, img_url).await?;
    render_rank(&mut img, ch);
    render_skill(&mut img, ch);
    render_path(&mut img, ch);
    render_user(&mut img, plr);
    render_lc(&mut img, ch);
    render_stats(&mut img, ch);
    render_relics(&mut img, ch);

    Ok(DynamicImage::ImageRgba8(img))
}

async fn render_char_img(
    img: &mut RgbaImage,
    ch: &CharacterData,
    img_url: Option<&str>,
) -> Result<(), Box<dyn Error + 'static>> {
    let mut bg = create_light_block(650, 716);
    let im = match img_url {
        Some(v) => match get_img_from_url(v).await {
            Ok(v2) => v2,
            Err(_) => get_img_from_url(&get_asset_url(&ch.preview)).await?,
        },
        _ => get_img_from_url(&get_asset_url(&ch.preview)).await?,
    };

    let chara = resize_to_fill_top(&im, 650, 716, image::imageops::FilterType::Triangle);
    image::imageops::overlay(&mut bg, &chara, 0, 0);

    if let Ok(element_img) = get_img_from_url(&get_asset_url(&ch.element.icon)).await {
        let outline_width = 3;
        let outline_color = Rgba([26, 26, 26, 255]);

        let element_img_resized =
            element_img.resize_exact(115, 115, image::imageops::FilterType::Triangle);
        let mut outlined_img = create_light_block(115 + outline_width * 2, 115 + outline_width * 2);

        for y in 0..outlined_img.height() {
            for x in 0..outlined_img.width() {
                outlined_img.put_pixel(x, y, outline_color);
            }
        }
        image::imageops::overlay(
            &mut outlined_img,
            &element_img_resized.to_rgba8(),
            outline_width.into(),
            outline_width.into(),
        );

        image::imageops::overlay(&mut bg, &outlined_img, 19, 108);
    }

    let mask = create_rounded_mask((bg.width(), bg.height()), 15).convert();
    apply_mask(&mut bg, &mask);

    image::imageops::overlay(img, &bg, 30, 30);

    Ok(())
}

fn apply_mask(img: &mut RgbaImage, mask: &RgbaImage) {
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let alpha = mask.get_pixel(x, y)[3];
        pixel.0[3] = alpha;
    }
}

// Placeholder functions for rendering other components
fn render_rank(img: &mut RgbaImage, _ch: &CharacterData) {}
fn render_skill(img: &mut RgbaImage, _ch: &CharacterData) {}
fn render_path(img: &mut RgbaImage, _ch: &CharacterData) {}
fn render_user(img: &mut RgbaImage, _plr: &PlayerData) {}
fn render_lc(img: &mut RgbaImage, _ch: &CharacterData) {}
fn render_stats(img: &mut RgbaImage, _ch: &CharacterData) {}
fn render_relics(img: &mut RgbaImage, _ch: &CharacterData) {}
