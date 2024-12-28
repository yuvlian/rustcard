use crate::asset_n_cfg::consts::ROUND_MASK_RADIUS;
use crate::utils::draw_blocks::draw_light_block;
use crate::utils::mask::{apply_mask, draw_rounded_mask};
use ab_glyph::{Font, PxScale};
use image::{Rgba, RgbaImage, imageops::overlay as ops_overlay};
use imageproc::drawing::draw_text_mut;
use mihomo4::PlayerData;

pub fn render(img: &mut RgbaImage, plr: PlayerData, font: &impl Font) {
    let mut bg = draw_light_block(475, 100);
    let plr_name_pos = (30, 13);
    let plr_name_scale = PxScale::from(35f32);
    let plr_name_color = Rgba([255, 255, 255, 255]);
    let plr_uid_pos = (30, 48);

    draw_text_mut(
        &mut bg,
        plr_name_color,
        plr_name_pos.0,
        plr_name_pos.1,
        plr_name_scale,
        font,
        &format!("USER: {}", plr.nickname),
    );

    draw_text_mut(
        &mut bg,
        plr_name_color,
        plr_uid_pos.0,
        plr_uid_pos.1,
        plr_name_scale,
        font,
        &format!("UID: {}", plr.uid),
    );

    let mask = draw_rounded_mask((bg.width(), bg.height()), ROUND_MASK_RADIUS);

    apply_mask(&mut bg, &mask);

    ops_overlay(img, &bg, 30 + 157 + 18, 30 + 716 + 18);
}
