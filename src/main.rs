//! `banbu` is the command line tool that generates banners from a YAML file.
//! It relies on the [banner_builder] library.
//! You should be able to find examples on the [Banner Builder](https://banner-builder.code-maven.com/)
//! web site and in the `site/examples` directory.

use clap::Parser;
use std::error::Error;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = env!("CARGO_BIN_NAME"), version)]
struct Cli {
    /// Path to the YAML configuration file
    yaml_file: PathBuf,

    /// Path to the output PNG image file
    image_file: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    //! The main function of the `banbu` command line tool.
    simple_logger::init_with_env()?;
    log::info!("Starting banner builder");

    let args = Cli::parse();

    let banner: banner_builder::Banner = banner_builder::read_yaml_file(&args.yaml_file);
    let root = args.yaml_file.parent().unwrap();
    banner_builder::draw_image(&banner, root, &args.image_file);

    Ok(())
}
