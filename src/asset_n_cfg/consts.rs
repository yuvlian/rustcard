use image::Rgba;
use image::imageops::FilterType;

pub const ROUND_MASK_RADIUS: u32 = 15;

pub const DARK_RGBA: Rgba<u8> = Rgba([26, 26, 26, 255]);
pub const LIGHT_RGBA: Rgba<u8> = Rgba([41, 41, 41, 255]);

pub const IMAGE_RESIZE_FILTER: FilterType = FilterType::Triangle;

pub const IMG_OUTLINE_THICKNESS: u32 = 1;
pub const TEXT_OUTLINE_THICKNESS: i32 = 3;

pub const ASSET_FOLDER_PATH: &str = "assets/";
pub const CARD_OUTPUT_PATH: &str = "out/";
