use image::{GenericImageView, Rgba, RgbaImage};
use imageproc::drawing::{draw_text_mut, text_size};
use rusttype::{Font, Scale};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct Embed {
    pub file: String,
    pub x: u32,
    pub y: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Text {
    pub text: String,
    pub x: u32,
    pub y: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Banner {
    pub width: u32,
    pub height: u32,
    pub text: String,

    #[serde(default = "default_white")]
    pub background_color: String,

    #[serde(default = "default_embed")]
    pub embed: Vec<Embed>,

    #[serde(default = "default_lines")]
    pub lines: Vec<Text>,
}

fn default_white() -> String {
    "FFFFFF".to_string()
}

fn default_embed() -> Vec<Embed> {
    vec![]
}

fn default_lines() -> Vec<Text> {
    vec![]
}

pub fn draw_image(banner: &Banner, root: &Path, path: &PathBuf) -> bool {
    log::info!("draw_image {path:?}");

    let limit = 90;
    if banner.text.len() > limit {
        log::warn!("Text is over the arbitrary limit of {limit} characters. Not generating.");
        return false;
    }

    let mut image = create_image(banner);

    for emb in &banner.embed {
        image = embed_image(image, &root.join(&emb.file), emb.x, emb.y);
    }

    //"/snap/cups/980/usr/share/fonts/truetype/freefont/FreeSans.ttf"
    log::info!("add text {:?}", banner.text);

    let font =
        Vec::from(include_bytes!("/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf") as &[u8]);
    let font = Font::try_from_vec(font).unwrap();

    add_centralized_text(&banner.text, &font, banner.height, banner.width, &mut image);

    let red = 50_u8;
    let green = 50;
    let blue = 0;
    let alpha = 255;

    let intended_text_height = 24.4;
    let scale = Scale {
        x: intended_text_height * 2.0,
        y: intended_text_height,
    };

    for line in &banner.lines {
        draw_text_mut(
            &mut image,
            Rgba([red, green, blue, alpha]),
            line.x as i32,
            line.y as i32,
            scale,
            &font,
            &line.text,
        );
    }

    image.save(path).unwrap();

    true
}

fn add_centralized_text(
    text: &str,
    font: &Font<'_>,
    banner_height: u32,
    max_width: u32,
    image: &mut image::ImageBuffer<Rgba<u8>, Vec<u8>>,
) {
    let intended_text_height = 24.4;
    let scale = Scale {
        x: intended_text_height * 2.0,
        y: intended_text_height,
    };

    // color of the text
    let red = 50_u8;
    let green = 50;
    let blue = 0;
    let alpha = 255;

    // get the size of the text and calculate the x, y coordinate where to start to be center aligned
    // both horizontally and vertically
    let width = 30;
    let lines = textwrap::wrap(text, width);
    let padding: u32 = 10;
    let (_text_width, text_height) = text_size(scale, font, text);
    let line_height = padding + text_height as u32;
    let start_row = (banner_height / 2) - line_height * (lines.len() as u32) / 2;
    //println!("start_row: {}", start_row);

    for (idx, line) in lines.iter().enumerate() {
        let (text_width, _text_height) = text_size(scale, font, line);
        //println!("Text size: {}x{}", text_width, text_height);
        //println!("banner width: {}  text width: {}", banner.width, text_width);
        let text_start_x = (max_width - text_width as u32) / 2;
        let text_start_y = start_row + (idx as u32) * line_height;

        draw_text_mut(
            image,
            Rgba([red, green, blue, alpha]),
            text_start_x as i32,
            text_start_y as i32,
            scale,
            font,
            line,
        );
    }
}

pub fn read_yaml_file(yaml_file: &PathBuf) -> Banner {
    log::info!("read_yaml_file: {yaml_file:?}");

    let banner: Banner = match std::fs::File::open(yaml_file) {
        Ok(file) => match serde_yaml::from_reader(file) {
            Ok(content) => content,
            Err(error) => {
                eprintln!("Error parsing '{yaml_file:?}', error: {error}");
                std::process::exit(1);
            }
        },
        Err(error) => {
            eprintln!("Could not open file '{yaml_file:?}', error: {error}");
            std::process::exit(1);
        }
    };
    banner
}

fn create_image(banner: &Banner) -> RgbaImage {
    log::info!("create_image");

    let mut image = RgbaImage::new(banner.width, banner.height);
    // set background color
    let red = u8::from_str_radix(&banner.background_color[0..=1], 16).unwrap();
    let green = u8::from_str_radix(&banner.background_color[2..=3], 16).unwrap();
    let blue = u8::from_str_radix(&banner.background_color[4..=5], 16).unwrap();
    let alpha = 255;

    for x in 0..banner.width {
        for y in 0..banner.height {
            *image.get_pixel_mut(x, y) = image::Rgba([red, green, blue, alpha]);
        }
    }

    image
}

fn embed_image(mut img: RgbaImage, infile: &PathBuf, start_x: u32, start_y: u32) -> RgbaImage {
    log::info!("embed_image from file {infile:?}");

    let logo = image::open(infile).unwrap();

    log::info!("Base image: width={}, height={}", img.width(), img.height());
    log::info!(
        "Embedding:  width={}, height={}",
        logo.width(),
        logo.height()
    );

    if start_x + logo.width() > img.width() {
        log::error!("Does not fit in width");
        return img;
    }
    if start_y + logo.height() > img.height() {
        log::error!("Does not fit in height");
        return img;
    }

    for x in 0..logo.width() {
        for y in 0..logo.height() {
            *img.get_pixel_mut(start_x + x, start_y + y) = logo.get_pixel(x, y);
        }
    }

    img
}
