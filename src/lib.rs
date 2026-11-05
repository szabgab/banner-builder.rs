//! **Banner Builder** was developed to make it easy to create thumbnails for YouTube videos,
//! banner images for Meetups, and for blog posts that would be included on various
//! social media platforms.
//!
//! It comes with a command line tool called [banbu](../banbu/index.html) and also a library that can be used in other Rust projects.

use ab_glyph::{FontRef, PxScale};
use image::DynamicImage;
use image::imageops::FilterType;
use image::{GenericImageView, Rgba, RgbaImage};
use imageproc::drawing::{draw_text_mut, text_size};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::path::Path;
use std::path::PathBuf;

/// Represent the background configuration of the banner
#[derive(Serialize, Deserialize, Debug)]
pub struct Background {
    /// Optional gradient background configuration
    #[serde(default)]
    pub gradient: Option<Gradient>,
}

/// Represent a gradient background
#[derive(Serialize, Deserialize, Debug)]
pub struct Gradient {
    /// Type of gradient. Currently only "linear" is supported.
    #[serde(default = "default_gradient_type", rename = "type")]
    pub gradient_type: String,

    /// Angle in degrees for linear gradients.
    #[serde(default = "default_gradient_angle")]
    pub angle: f32,

    /// Gradient stops represented by colors or explicit stop objects.
    pub stops: Vec<GradientStopInput>,
}

/// Represent a gradient stop with either a color string or an object
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum GradientStopInput {
    Color(String),
    Stop {
        color: String,
        #[serde(default)]
        position: Option<String>,
    },
}

#[derive(Debug)]
struct GradientStop {
    color: Rgba<u8>,
    position: f32,
}

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

    /// Optional structured background configuration
    #[serde(default)]
    pub background: Option<Background>,

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

fn default_gradient_type() -> String {
    "linear".to_string()
}

fn default_gradient_angle() -> f32 {
    90.0
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

    let mut image = match create_image(banner) {
        Ok(image) => image,
        Err(error) => {
            eprintln!("Error creating background: {error}");
            std::process::exit(1);
        }
    };

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
                eprintln!(
                    "In rtl=true line '{:?}', the text is right-aligned and thus x should be the right coordinat. (x: {} < text width: {})",
                    line.text.reverse(),
                    line.x,
                    text_width
                );
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

fn get_color_checked(color: &str) -> Result<image::Rgba<u8>, String> {
    if color.len() != 6 && color.len() != 8 {
        return Err(format!(
            "Invalid color '{color}'. Expected 6 or 8 hex characters."
        ));
    }

    if !color.chars().all(|ch| ch.is_ascii_hexdigit()) {
        return Err(format!(
            "Invalid color '{color}'. Expected only hex characters."
        ));
    }

    Ok(get_color(color))
}

fn parse_percent(position: &str) -> Result<f32, String> {
    if !position.ends_with('%') {
        return Err(format!(
            "Invalid gradient stop position '{position}'. Expected a percentage string like '25%'."
        ));
    }

    let numeric = position.trim_end_matches('%');
    let parsed = numeric.parse::<f32>().map_err(|error| {
        format!("Invalid gradient stop position '{position}'. Could not parse number: {error}")
    })?;
    if !(0.0..=100.0).contains(&parsed) {
        return Err(format!(
            "Invalid gradient stop position '{position}'. Percentage must be between 0% and 100%."
        ));
    }

    Ok(parsed / 100.0)
}

fn normalize_gradient_stops(stops: &[GradientStopInput]) -> Result<Vec<GradientStop>, String> {
    if stops.len() < 2 {
        return Err(format!(
            "Gradient requires at least 2 stops, received {}.",
            stops.len()
        ));
    }

    let mut colors = Vec::with_capacity(stops.len());
    let mut positions: Vec<Option<f32>> = Vec::with_capacity(stops.len());

    for stop in stops {
        match stop {
            GradientStopInput::Color(color) => {
                colors.push(get_color_checked(color)?);
                positions.push(None);
            }
            GradientStopInput::Stop { color, position } => {
                colors.push(get_color_checked(color)?);
                positions.push(match position {
                    Some(position) => Some(parse_percent(position)?),
                    None => None,
                });
            }
        }
    }

    if positions.iter().all(Option::is_none) {
        let max_index = (positions.len() - 1) as f32;
        for (index, position) in positions.iter_mut().enumerate() {
            *position = Some(index as f32 / max_index);
        }
    } else {
        if positions[0].is_none() {
            positions[0] = Some(0.0);
        }
        let last_index = positions.len() - 1;
        if positions[last_index].is_none() {
            positions[last_index] = Some(1.0);
        }

        let mut index = 0;
        while index < positions.len() {
            if positions[index].is_none() {
                let start = index - 1;
                let mut end = index + 1;
                while end < positions.len() && positions[end].is_none() {
                    end += 1;
                }

                let start_pos = positions[start].unwrap();
                let end_pos = positions[end].unwrap();
                if end_pos < start_pos {
                    return Err(format!(
                        "Gradient stop positions must be non-decreasing. Found {}% before {}%.",
                        end_pos * 100.0,
                        start_pos * 100.0
                    ));
                }
                let span = (end - start) as f32;
                for (current, position) in
                    positions.iter_mut().enumerate().take(end).skip(start + 1)
                {
                    let ratio = (current - start) as f32 / span;
                    *position = Some(start_pos + (end_pos - start_pos) * ratio);
                }
                index = end;
            } else {
                index += 1;
            }
        }
    }

    let mut previous = -1.0_f32;
    let mut normalized = Vec::with_capacity(colors.len());
    for (index, color) in colors.into_iter().enumerate() {
        let position = positions[index].unwrap();
        if position < previous {
            return Err(format!(
                "Gradient stop positions must be non-decreasing. Position at index {index} is smaller than a previous position."
            ));
        }
        previous = position;
        normalized.push(GradientStop { color, position });
    }

    Ok(normalized)
}

fn interpolate_channel(start: u8, end: u8, ratio: f32) -> u8 {
    ((start as f32) + ((end as f32) - (start as f32)) * ratio).round() as u8
}

fn sample_gradient_color(stops: &[GradientStop], t: f32) -> Rgba<u8> {
    if t <= stops[0].position {
        return stops[0].color;
    }
    if t >= stops[stops.len() - 1].position {
        return stops[stops.len() - 1].color;
    }

    for pair in stops.windows(2) {
        let start = &pair[0];
        let end = &pair[1];
        if t >= start.position && t <= end.position {
            let span = end.position - start.position;
            if span == 0.0 {
                return end.color;
            }
            let ratio = (t - start.position) / span;
            return Rgba([
                interpolate_channel(start.color[0], end.color[0], ratio),
                interpolate_channel(start.color[1], end.color[1], ratio),
                interpolate_channel(start.color[2], end.color[2], ratio),
                interpolate_channel(start.color[3], end.color[3], ratio),
            ]);
        }
    }

    stops[stops.len() - 1].color
}

fn draw_linear_gradient(
    image: &mut RgbaImage,
    width: u32,
    height: u32,
    angle_degrees: f32,
    stops: &[GradientStop],
) {
    let radians = angle_degrees.to_radians();
    let direction_x = radians.cos();
    let direction_y = radians.sin();

    let width_span = (width.saturating_sub(1)) as f32;
    let height_span = (height.saturating_sub(1)) as f32;
    let half_span = (direction_x.abs() * width_span + direction_y.abs() * height_span) / 2.0;
    let denominator = if half_span > 0.0 { half_span } else { 1.0 };
    let center_x = width_span / 2.0;
    let center_y = height_span / 2.0;

    for x in 0..width {
        for y in 0..height {
            let projected = ((x as f32 - center_x) * direction_x
                + (y as f32 - center_y) * direction_y)
                / denominator;
            let t = (0.5 + projected).clamp(0.0, 1.0);
            *image.get_pixel_mut(x, y) = sample_gradient_color(stops, t);
        }
    }
}

fn apply_gradient_background(
    image: &mut RgbaImage,
    width: u32,
    height: u32,
    gradient: &Gradient,
) -> Result<(), String> {
    if gradient.gradient_type != "linear" {
        return Err(format!(
            "Unsupported gradient type '{}'. Currently only 'linear' is supported.",
            gradient.gradient_type
        ));
    }

    let stops = normalize_gradient_stops(&gradient.stops)?;
    draw_linear_gradient(image, width, height, gradient.angle, &stops);
    Ok(())
}

fn create_image(banner: &Banner) -> Result<RgbaImage, String> {
    log::info!("create_image");

    let mut image = RgbaImage::new(banner.width, banner.height);
    if let Some(gradient) = banner
        .background
        .as_ref()
        .and_then(|background| background.gradient.as_ref())
    {
        apply_gradient_background(&mut image, banner.width, banner.height, gradient)?;
    } else {
        // set background color
        let color = get_color(&banner.background_color);
        for x in 0..banner.width {
            for y in 0..banner.height {
                *image.get_pixel_mut(x, y) = color;
            }
        }
    }

    Ok(image)
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
        return Err(Box::<dyn Error>::from(format!(
            "The image {infile:?} does not fit in width. start_x: {start_x} width: {} available: {}",
            logo.width(),
            img.width()
        )));
    }
    if start_y + logo.height() > img.height() {
        return Err(Box::<dyn Error>::from(format!(
            "The image {infile:?} does not fit in height. start_y: {start_y} height: {} available: {}",
            logo.height(),
            img.height()
        )));
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
