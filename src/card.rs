use crate::config::*;
use crate::util::*;
use image::buffer::ConvertBuffer;
use image::*;
use imageproc::drawing::draw_text_mut;
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

    let chara = resize_to_fill_and_stick_image_top_to_top(
        &im,
        650,
        716,
        image::imageops::FilterType::Triangle,
    );
    image::imageops::overlay(&mut bg, &chara, 0, 0);

    if let Ok(element_img) = get_img_from_url(&get_asset_url(&ch.element.icon)).await {
        let outline_color = Rgba([26, 26, 26, 255]);
        let outline_width = 2;
        let resize_width = 115;

        let element_img_resized = element_img
            .resize_exact(
                resize_width,
                resize_width,
                image::imageops::FilterType::Triangle,
            )
            .to_rgba8();

        let mut outlined_img =
            create_png_outline(&element_img_resized, outline_color, outline_width);

        imageops::overlay(
            &mut outlined_img,
            &element_img_resized,
            outline_width.into(),
            outline_width.into(),
        );

        imageops::overlay(&mut bg, &outlined_img, 19, 108);
    }

    // let font = get_font().ok_or("Font not found")?;
    let ab_font = get_ab_font();

    let name = &ch.name;
    let scale = 62.0;
    let position = (19, 19);
    let color = Rgba([26, 26, 26, 255]);
    let color2 = Rgba([255, 255, 255, 255]);
    let scale = ab_glyph::PxScale::from(82.0);
    let scale2 = ab_glyph::PxScale::from(62.0);
    // draw_text_mut(&mut bg, color, 19i32, 60i32, scale, &ab_font, name);
    // draw_text_mut(&mut bg, color2, 19i32, 60i32, scale2, &ab_font, name);
    draw_text_with_outline(&mut bg, name, position, &ab_font, scale2, color2, color, 5i32);
    // let level_text = format!("Lv. {}/{}", ch.level, ch.max_level());
    // let position = (22, 102);
    // let scale = 31.0;
    // draw_text(
    //     &mut bg,
    //     &level_text,
    //     position,
    //     &font,
    //     scale,
    //     color,
    //     stroke_color,
    //     stroke_width,
    // );

    let mask = create_rounded_mask((bg.width(), bg.height()), 25).convert();
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
