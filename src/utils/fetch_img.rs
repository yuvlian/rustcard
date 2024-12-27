use image::{DynamicImage, ImageFormat, load_from_memory_with_format};
use mihomo4::Client;
use std::error::Error;

const ASSET_BASE_URL: &str = "https://raw.githubusercontent.com/Mar-7th/StarRailRes/master/";

pub fn get_full_asset_url(path: &str) -> String {
    format!("{}{}", ASSET_BASE_URL, path)
}

pub async fn get_img_from_url(url: &str) -> Result<DynamicImage, Box<dyn Error>> {
    let response = Client::new().get(url).send().await?;
    let bytes = response.bytes().await?;

    let img = load_from_memory_with_format(&bytes, ImageFormat::Png)?;

    Ok(img)
}
