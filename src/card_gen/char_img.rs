use crate::asset_n_cfg::consts::{
    DARK_RGBA, IMAGE_RESIZE_FILTER, IMG_OUTLINE_THICKNESS, ROUND_MASK_RADIUS,
    TEXT_OUTLINE_THICKNESS,
};
use crate::utils::draw_blocks::draw_light_block;
use crate::utils::fetch_img::{get_full_asset_url, get_img_from_url};
use crate::utils::img_size_manip::resize_to_fill_and_stick_image_top_to_top;
use crate::utils::mask::{apply_mask, draw_rounded_mask};
use crate::utils::outlined_draw::{draw_png_with_outline, draw_text_with_outline};
use ab_glyph::{Font, PxScale};
use image::{Rgba, RgbaImage, imageops::overlay as ops_overlay};
use mihomo4::{CharacterData, Client};
use std::error::Error;

pub async fn render(
    img: &mut RgbaImage,
    ch: &CharacterData,
    cl: &Client,
    img_url: Option<&str>,
    font: &impl Font,
) -> Result<(), Box<dyn Error>> {
    let mut bg = draw_light_block(650, 716);

    let im = match img_url {
        Some(v) => match get_img_from_url(v, cl).await {
            Ok(v2) => v2,
            Err(_) => get_img_from_url(&get_full_asset_url(&ch.preview), cl).await?,
        },
        _ => get_img_from_url(&get_full_asset_url(&ch.preview), cl).await?,
    };

    let chara = resize_to_fill_and_stick_image_top_to_top(&im, 650, 716, IMAGE_RESIZE_FILTER);

    ops_overlay(&mut bg, &chara, 0, 0);

    if let Ok(element_img) = get_img_from_url(&get_full_asset_url(&ch.element.icon), cl).await {
        let outline_color = DARK_RGBA;
        let outline_width = IMG_OUTLINE_THICKNESS;
        let resize_width = 115;

        let element_img_resized = element_img
            .resize_exact(resize_width, resize_width, IMAGE_RESIZE_FILTER)
            .to_rgba8();

        let mut outlined_img =
            draw_png_with_outline(&element_img_resized, outline_color, outline_width);

        ops_overlay(
            &mut outlined_img,
            &element_img_resized,
            outline_width.into(),
            outline_width.into(),
        );

        ops_overlay(&mut bg, &outlined_img, 19, 108);
    }

    let ch_name = &ch.name;
    let ch_name_pos = (18i32, 9i32);
    let ch_name_scale = PxScale::from(62f32);
    let ch_name_color = Rgba([255, 255, 255, 255]);
    let ch_name_outline_thickness = TEXT_OUTLINE_THICKNESS;
    let ch_name_outline_color = DARK_RGBA;

    // draw name
    draw_text_with_outline(
        font,
        &mut bg,
        ch_name,
        ch_name_pos,
        ch_name_scale,
        ch_name_color,
        ch_name_outline_color,
        ch_name_outline_thickness,
    );

    let ch_lv_txt = format!("Lv. {}/{}", ch.level, ch.max_level());
    let ch_lv_txt_pos = (20, 72);
    let ch_lv_txt_scale = PxScale::from(31.0);

    // draw lvl
    draw_text_with_outline(
        font,
        &mut bg,
        &ch_lv_txt,
        ch_lv_txt_pos,
        ch_lv_txt_scale,
        ch_name_color,
        ch_name_outline_color,
        ch_name_outline_thickness,
    );

    let mask = draw_rounded_mask((bg.width(), bg.height()), ROUND_MASK_RADIUS);

    apply_mask(&mut bg, &mask);

    ops_overlay(img, &bg, 30, 30);

    Ok(())
}
