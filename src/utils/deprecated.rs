// i like keeping shitcode around,
// because they make me feel good when i compare them
// with my newer codes.
// well, to be fair, its not like the new one is any better.

// pub fn draw_text(
//     img: &mut RgbaImage,
//     text: &str,
//     position: (i32, i32),
//     font: &rusttype::Font,
//     scale: f32,
//     color: Rgba<u8>,
//     stroke_color: Rgba<u8>,
//     stroke_width: f32,
// ) {
//     let scale = Scale { x: scale, y: scale };
//     let offset = point(position.0 as f32, position.1 as f32);

//     let glyphs: Vec<PositionedGlyph> = font.layout(text, scale, offset).collect();

//     for glyph in glyphs {
//         if let Some(bounding_box) = glyph.pixel_bounding_box() {
//             if stroke_width > 0.0 {
//                 glyph.draw(|x, y, v| {
//                     let x2 = (x as i32 + bounding_box.min.x) as u32;
//                     let y2 = (y as i32 + bounding_box.min.y) as u32;
//                     if v > 0.0 {
//                         if x2 < img.width() && y2 < img.height() {
//                             let pixel = img.get_pixel_mut(x2, y2);
//                             let blended = blend_text_pixel(*pixel, stroke_color);
//                             *pixel = blended;
//                         }
//                     }
//                 });
//             }

//             glyph.draw(|x, y, v| {
//                 let x2 = (x as i32 + bounding_box.min.x) as u32;
//                 let y2 = (y as i32 + bounding_box.min.y) as u32;
//                 if v > 0.0 {
//                     if x2 < img.width() && y2 < img.height() {
//                         let pixel = img.get_pixel_mut(x2, y2);
//                         let blended = blend_text_pixel(*pixel, color);
//                         *pixel = blended;
//                     }
//                 }
//             });
//         }
//     }
// }

// fn blend_text_pixel(bottom: Rgba<u8>, top: Rgba<u8>) -> Rgba<u8> {
//     let alpha = top[3] as f32 / 255.0;
//     let inv_alpha = 1.0 - alpha;

//     Rgba([
//         (top[0] as f32 * alpha + bottom[0] as f32 * inv_alpha) as u8,
//         (top[1] as f32 * alpha + bottom[1] as f32 * inv_alpha) as u8,
//         (top[2] as f32 * alpha + bottom[2] as f32 * inv_alpha) as u8,
//         255,
//     ])
// }

// pub fn create_output_from_bytes() {
//     use std::io::Write;
//     let file_bytes = format!("{:#?};", include_bytes!("../../assets/NoLC.png"));
//     let mut output_file = std::fs::File::create("output.txt").expect("Unable to create file");
//     writeln!(output_file, "{}", file_bytes).expect("Unable to write data to file");
// }