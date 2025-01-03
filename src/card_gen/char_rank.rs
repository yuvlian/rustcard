use crate::asset_n_cfg::consts::{IMAGE_RESIZE_FILTER, ROUND_MASK_RADIUS};
use crate::utils::draw_blocks::draw_light_block;
use crate::utils::fetch_img::{get_full_asset_url, get_img_from_url};
use crate::utils::img_size_manip::resize_to_fit;
use crate::utils::mask::{apply_mask, draw_rounded_mask};
use image::{Rgba, RgbaImage, imageops::overlay as ops_overlay};
use mihomo4::{CharacterData, Client};
use std::error::Error;

pub async fn render(
    img: &mut RgbaImage,
    ch: &CharacterData,
    cl: &Client,
) -> Result<(), Box<dyn Error>> {
    let mut rank_bar = draw_light_block(124, 487);

    for (i, icon_url) in ch.rank_icons.iter().enumerate() {
        let rank_img = get_img_from_url(&get_full_asset_url(icon_url), cl).await?;
        let rank_resized = resize_to_fit(&rank_img, 71, 71, IMAGE_RESIZE_FILTER);
        let rank_resized = rank_resized.to_rgba8();

        let to_paste = if i < ch.rank.into() {
            rank_resized
        } else {
            let mut overlay = RgbaImage::new(71, 71);
            for (x, y, pixel) in rank_resized.enumerate_pixels() {
                let new_pixel = match pixel[3] {
                    _ if pixel[3] < 35 => pixel,
                    _ => &Rgba([pixel[0], pixel[1], pixel[2], 35]),
                };
                overlay.put_pixel(x, y, *new_pixel);
            }
            overlay
        };

        let x_offset = 26;
        let y_offset = 7 + i * (71 + 10);
        ops_overlay(&mut rank_bar, &to_paste, x_offset as i64, y_offset as i64);
    }

    let rank_bar_mask = draw_rounded_mask((rank_bar.width(), rank_bar.height()), ROUND_MASK_RADIUS);
    apply_mask(&mut rank_bar, &rank_bar_mask);

    let x_position = 30 + 650 + 18;
    let y_position = 30;

    ops_overlay(img, &rank_bar, x_position as i64, y_position as i64);

    Ok(())
}
