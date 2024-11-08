//! **Banner Builder** was developed to make it easy to create thumbnails for YouTube videos,
//! banner images for Meetups, and for blog posts that would be included on various
//! social media platforms.
//!
//! It comes with a command line tool called [banbu](../banbu/index.html) and also a library that can be used in other Rust projects.

use ab_glyph::{FontRef, PxScale};
use image::imageops::FilterType;
use image::DynamicImage;
use image::{GenericImageView, Rgba, RgbaImage};
use imageproc::drawing::{draw_text_mut, text_size};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::path::Path;
use std::path::PathBuf;

/// Represent an image to be embedded in the banner
#[derive(Serialize, Deserialize, Debug)]
pub struct Embed {
    pub file: String,
    pub x: u32,
    pub y: u32,
    pub width: Option<u32>,
}

/// Represent a line of text to be displayed on the banner
#[derive(Serialize, Deserialize, Debug)]
pub struct Text {
    pub text: String,
    pub x: u32,
    pub y: u32,

    // if true, the text is right-to-left
    #[serde(default = "default_false")]
    pub rtl: bool,

    /// RGB color of the text (in hex format)
    #[serde(default = "default_black")]
    pub color: String,

    #[serde(default = "default_font_size")]
    pub size: i32,
}

/// Struct to hold all the information about the banner
///
/// See the fields for more information.
#[derive(Serialize, Deserialize, Debug)]
pub struct Banner {
    /// width of the banner
    pub width: u32,
    /// height of the banner
    pub height: u32,
    /// text to be displayed on the banner (better use the lines for multi-line text)
    pub text: String,

    /// font size of the text
    #[serde(default = "default_font_size")]
    pub size: i32,

    /// background color of the banner (in hex format)
    #[serde(default = "default_white")]
    pub background_color: String,

    /// List of images to be embedded in the banner
    #[serde(default = "default_embed")]
    pub embed: Vec<Embed>,

    /// List of lines to be displayed on the banner
    #[serde(default = "default_lines")]
    pub lines: Vec<Text>,
}

fn default_false() -> bool {
    false
}

fn default_font_size() -> i32 {
    24
}

fn default_black() -> String {
    "000000FF".to_string()
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

// fn reverse_string(text: &str) -> String {
//     text.chars().rev().collect::<String>()
// }

trait Reverse {
    fn reverse(&self) -> String;
}

impl Reverse for str {
    fn reverse(&self) -> String {
        self.chars().rev().collect::<String>()
    }
}

/// Draw the image based on the information in the `Banner` struct
///
/// * banner: the `Banner` struct that holds all the information
/// * root: the root directory where the images are located
/// * path: where to save the generated image
pub fn draw_image(banner: &Banner, root: &Path, path: &PathBuf) -> bool {
    log::info!("draw_image {path:?}");

    let limit = 90;
    if banner.text.len() > limit {
        log::warn!("Text is over the arbitrary limit of {limit} characters. Not generating.");
        return false;
    }

    let mut image = create_image(banner);

    for emb in &banner.embed {
        embed_image(&mut image, &root.join(&emb.file), emb.x, emb.y, emb.width).unwrap();
    }

    //"/snap/cups/980/usr/share/fonts/truetype/freefont/FreeSans.ttf"
    log::info!("add text {:?}", banner.text);

    let font = include_bytes!("/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf");
    let font = FontRef::try_from_slice(font).unwrap();

    add_centralized_text(
        &banner.text,
        banner.size,
        &font,
        banner.height,
        banner.width,
        &mut image,
    );

    add_text_lines(banner, &mut image, font);

    image.save(path).unwrap();

    true
}

fn add_text_lines(
    banner: &Banner,
    image: &mut image::ImageBuffer<Rgba<u8>, Vec<u8>>,
    font: FontRef,
) {
    for line in &banner.lines {
        let scale = PxScale::from(line.size as f32);
        let (text_width, _text_height) = text_size(scale, &font, &line.text);

        let (text, x, y) = if line.rtl {
            if line.x < text_width {
                eprintln!("In rtl=true line '{:?}', the text is right-aligned and thus x should be the right coordinat. (x: {} < text width: {})", line.text.reverse(), line.x, text_width);
                std::process::exit(1);
            }

            (line.text.reverse(), line.x - text_width, line.y)
        } else {
            (line.text.clone(), line.x, line.y)
        };

        draw_text_mut(
            image,
            get_color(&line.color),
            x as i32,
            y as i32,
            scale,
            &font,
            &text,
        );
    }
}

fn add_centralized_text(
    text: &str,
    size: i32,
    font: &FontRef,
    banner_height: u32,
    max_width: u32,
    image: &mut image::ImageBuffer<Rgba<u8>, Vec<u8>>,
) {
    let scale = PxScale::from(size as f32);

    // color of the text
    let red = 0_u8;
    let green = 0;
    let blue = 0;
    let alpha = 255;

    // get the size of the text and calculate the x, y coordinate where to start to be center aligned
    // both horizontally and vertically
    let width = 30;
    let lines = textwrap::wrap(text, width);
    let padding: u32 = 10;
    let (_text_width, text_height) = text_size(scale, &font, text);
    let line_height = padding + text_height;
    let start_row = (banner_height / 2) - line_height * (lines.len() as u32) / 2;
    //println!("start_row: {}", start_row);

    for (idx, line) in lines.iter().enumerate() {
        let (text_width, _text_height) = text_size(scale, &font, line);
        //println!("Text size: {}x{}", text_width, text_height);
        //println!("banner width: {}  text width: {}", banner.width, text_width);
        let text_start_x = (max_width - text_width) / 2;
        let text_start_y = start_row + (idx as u32) * line_height;

        draw_text_mut(
            image,
            Rgba([red, green, blue, alpha]),
            text_start_x as i32,
            text_start_y as i32,
            scale,
            &font,
            line,
        );
    }
}

/// Read the YAML file and return the `Banner` struct
///
/// * yaml_file: the path to the YAML file
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

fn get_color(color: &str) -> image::Rgba<u8> {
    let red = u8::from_str_radix(&color[0..=1], 16).unwrap();
    let green = u8::from_str_radix(&color[2..=3], 16).unwrap();
    let blue = u8::from_str_radix(&color[4..=5], 16).unwrap();
    let alpha = if color.len() == 6 {
        255
    } else {
        u8::from_str_radix(&color[6..=7], 16).unwrap()
    };

    image::Rgba([red, green, blue, alpha])
}

fn create_image(banner: &Banner) -> RgbaImage {
    log::info!("create_image");

    let mut image = RgbaImage::new(banner.width, banner.height);
    // set background color
    let color = get_color(&banner.background_color);

    for x in 0..banner.width {
        for y in 0..banner.height {
            *image.get_pixel_mut(x, y) = color;
        }
    }

    image
}

fn resize_image(img: DynamicImage, width: u32) -> DynamicImage {
    let height = width * img.height() / img.width();
    let filter = FilterType::Nearest;
    img.resize(width, height, filter)
}

fn embed_image(
    img: &mut image::ImageBuffer<Rgba<u8>, Vec<u8>>,
    infile: &PathBuf,
    start_x: u32,
    start_y: u32,
    width: Option<u32>,
) -> Result<(), Box<dyn Error>> {
    log::info!("embed_image from file {infile:?}");

    let logo = image::open(infile)?;
    let logo = match width {
        Some(width) => resize_image(logo, width),
        None => logo,
    };

    log::info!("Base image: width={}, height={}", img.width(), img.height());
    log::info!(
        "Embedding:  width={}, height={}",
        logo.width(),
        logo.height()
    );

    if start_x + logo.width() > img.width() {
        return Err(Box::<dyn Error>::from(format!("The image {infile:?} does not fit in width. start_x: {start_x} width: {} available: {}", logo.width(), img.width())));
    }
    if start_y + logo.height() > img.height() {
        return Err(Box::<dyn Error>::from(format!("The image {infile:?} does not fit in height. start_y: {start_y} height: {} available: {}", logo.height(), img.height())));
    }

    for x in 0..logo.width() {
        for y in 0..logo.height() {
            let px = logo.get_pixel(x, y);
            // use alpha channel as mask
            if px[3] == 255 {
                *img.get_pixel_mut(start_x + x, start_y + y) = logo.get_pixel(x, y);
            }
        }
    }

    Ok(())
}
