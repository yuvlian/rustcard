use crate::asset_n_cfg::consts::{
    DARK_RGBA, IMAGE_RESIZE_FILTER, ROUND_MASK_RADIUS, TEXT_OUTLINE_THICKNESS,
};
use crate::utils::draw_blocks::draw_light_block;
use crate::utils::fetch_img::{get_full_asset_url, get_img_from_url};
use crate::utils::img_size_manip::resize_to_fit;
use crate::utils::mask::{apply_mask, draw_rounded_mask};
use crate::utils::outlined_draw::draw_text_with_outline;
use ab_glyph::{Font, PxScale};
use image::{Rgba, RgbaImage, imageops::overlay as ops_overlay};
use mihomo4::{CharacterData, Client};
use std::error::Error;

pub async fn render(
    img: &mut RgbaImage,
    ch: &CharacterData,
    cl: &Client,
    font: &impl Font,
) -> Result<(), Box<dyn Error>> {
    let mut bg = draw_light_block(124, 330);

    for (i, s) in Vec::from(["Basic ATK", "Talent", "Skill", "Ultimate"])
        .iter()
        .enumerate()
    {
        if let Some(skill) = ch.skills.iter().find(|x| &x.type_text == s) {
            let skill_img = get_img_from_url(&get_full_asset_url(&skill.icon), cl).await?;

            let mut resized_skill_img =
                resize_to_fit(&skill_img, 71, 71, IMAGE_RESIZE_FILTER).to_rgba8();

            let skill_level_text = skill.level.to_string();
            let text_scale = PxScale::from(35.0);
            let text_color = Rgba([255, 255, 255, 255]);
            let outline_color = DARK_RGBA;

            let x_offset: i32 = if skill_level_text.len() == 1 {
                71 - 25
            } else {
                71 - 35
            };

            let y_offset: i32 = 71 - 35;

            draw_text_with_outline(
                font,
                &mut resized_skill_img,
                &skill_level_text,
                (x_offset, y_offset),
                text_scale,
                text_color,
                outline_color,
                TEXT_OUTLINE_THICKNESS,
            );

            let position = (26, 9 + i as i64 * (71 + 10));

            ops_overlay(&mut bg, &resized_skill_img, position.0, position.1);
        } else {
            println!("->> Index: {}, No skill found for {}", i, s);
        }
    }

    let bg_mask = draw_rounded_mask((124, 330), ROUND_MASK_RADIUS);
    apply_mask(&mut bg, &bg_mask);

    let bg_position = (30 + 650 + 18, 30 + 487 + 18);
    ops_overlay(img, &bg, bg_position.0, bg_position.1);

    Ok(())
}
