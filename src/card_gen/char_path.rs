use crate::asset_n_cfg::consts::{IMAGE_RESIZE_FILTER, ROUND_MASK_RADIUS};
use crate::utils::draw_blocks::draw_light_block;
use crate::utils::fetch_img::{get_full_asset_url, get_img_from_url};
use crate::utils::img_size_manip::resize_to_fit;
use crate::utils::mask::{apply_mask, draw_rounded_mask};
use image::{RgbaImage, imageops::overlay as ops_overlay};
use mihomo4::{CharacterData, Client};
use std::error::Error;

pub async fn render(
    img: &mut RgbaImage,
    ch: &CharacterData,
    cl: &Client,
) -> Result<(), Box<dyn Error>> {
    let mut path_bar = draw_light_block(157, 100);

    let path_img = get_img_from_url(&get_full_asset_url(&ch.path.icon), cl).await?;
    let path_resized = resize_to_fit(&path_img, 79, 79, IMAGE_RESIZE_FILTER);
    let path_resized = path_resized.to_rgba8();

    ops_overlay(&mut path_bar, &path_resized, 39, 10);

    let path_bar_mask = draw_rounded_mask((path_bar.width(), path_bar.height()), ROUND_MASK_RADIUS);

    apply_mask(&mut path_bar, &path_bar_mask);

    let x_position = 30;
    let y_position = 30 + 716 + 18;

    ops_overlay(img, &path_bar, x_position as i64, y_position as i64);

    Ok(())
}
