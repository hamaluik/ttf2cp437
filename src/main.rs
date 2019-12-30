#[macro_use] extern crate clap;

use std::path::PathBuf;

mod cli;

fn main() -> Result<(), String> {
    let matches = cli::build_cli().get_matches();

    let font_path: PathBuf = clap::value_t!(matches, "FONT", PathBuf).unwrap_or_else(|e| e.exit());
    let char_height: usize = clap::value_t!(matches, "HEIGHT", usize).unwrap_or_else(|e| e.exit());
    let scale: usize = matches.value_of("SCALE").map(std::str::FromStr::from_str).transpose().map_err(|e| format!("scale is not an unsigned integer: {:?}", e))?.unwrap_or(1);
    let char_height = char_height * scale;

    if !font_path.exists() {
        return Err(format!("The font file `{}` doesn't exist!", font_path.display()));
    }
    if !font_path.is_file() {
        return Err(format!("The font file `{}` isn't a file!", font_path.display()));
    }
    if !font_path.file_stem().is_some() {
        return Err(format!("The font file `{}` must have a name!", font_path.display()));
    }
    let is_ttf = if let Some("ttf") = font_path.extension().map(std::ffi::OsStr::to_str).flatten() { true } else { false };
    if !is_ttf {
        return Err(format!("The font file `{}` should have a `.ttf` extension!", font_path.display()));
    }

    use image::{DynamicImage, Rgba};
    use rusttype::{point, Font, Scale};

    let font_data = std::fs::read(&font_path).map_err(|e| format!("Failed to open `{}` for reading: {:?}", font_path.display(), e))?;
    let font = Font::from_bytes(&font_data).map_err(|e| format!("Failed to construct font from `{}`, are you sure it's a valid font file?: {:?}", font_path.display(), e))?;

    let font_scale = Scale::uniform(char_height as f32);
    let v_metrics = font.v_metrics(font_scale);
    let image_height = (char_height * 16).next_power_of_two();
    let image_width = image_height;

    let lines = vec![
        r##" ☺☻♥♦♣♠•◘○◙♂♀♪♫☼"##,
        r##"►◄↕‼¶§▬↨↑↓→←∟↔▲▼"##,
        r##" !"#$%&'()*+,-./"##,
        r##"0123456789:;<=>?"##,
        r##"@ABCDEFGHIJKLMNO"##,
        r##"PQRSTUVWXYZ[\]^_"##,
        r##"`abcdefghijklmno"##,
        r##"pqrstuvwxyz{|}~⌂"##,
        r##"ÇüéâäàåçêëèïîìÄÅ"##,
        r##"ÉæÆôöòûùÿÖÜ¢£¥₧ƒ"##,
        r##"áíóúñÑªº¿⌐¬½¼¡«»"##,
        r##"░▒▓│┤╡╢╖╕╣║╗╝╜╛┐"##,
        r##"└┴┬├─┼╞╟╚╔╩╦╠═╬╧"##,
        r##"╨╤╥╙╘╒╓╫╪┘┌█▄▌▐▀"##,
        r##"αßΓπΣσµτΦΘΩδ∞φε∩"##,
        r##"≡±≥≤⌠⌡÷≈°∙·√ⁿ²■ "##,
    ];

    let glyphs: Vec<rusttype::PositionedGlyph> = lines.iter().enumerate().flat_map(|(row, line)| {
        font.layout(line, font_scale, point(0.0f32, (char_height * row) as f32 + v_metrics.ascent))
    }).collect();
    let mut image = DynamicImage::new_rgba8(image_width as u32, image_height as u32).to_rgba();

    for glyph in glyphs {
        if let Some(bounding_box) = glyph.pixel_bounding_box() {
            glyph.draw(|x, y, v| {
                image.put_pixel(
                    x + bounding_box.min.x as u32,
                    y + bounding_box.min.y as u32,
                    Rgba([255, 255, 255, (v * 255.0) as u8]),
                )
            });
        }
    }

    let file_stem = font_path.file_stem().map(std::ffi::OsStr::to_str).flatten().expect("file stem");
    let out_path = if let Some(parent) = font_path.parent() {
        parent.join(format!("{}_{}x.png", file_stem, scale))
    }
    else {
        PathBuf::from(format!("{}_{}x.png", file_stem, scale))
    };

    image.save(&out_path).map_err(|e| format!("Failed to save output to `{}`: {:?}", out_path.display(), e))?;
    println!("Saved cp437 version to `{}`!", out_path.display());

    return Ok(());
}
