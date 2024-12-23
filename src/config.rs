pub const CARD_OUTPUT_PATH: &str = "../out/";

pub fn get_font() -> Option<rusttype::Font<'static>> {
    let x = include_bytes!("../assets/WorkSans-SemiBold.ttf");
    rusttype::Font::try_from_bytes(x)
}

pub fn get_no_lc_img() -> Result<image::RgbaImage, Box<dyn std::error::Error>> {
    let v = image::open("../assets/NoLC.png")?.to_rgba8();
    Ok(v)
}
