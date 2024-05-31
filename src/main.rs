use std::error::Error;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::init_with_env()?;
    log::info!("Starting banner builder");

    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        return Err(Box::<dyn Error>::from(format!(
            "{} config.yaml image.png",
            &args[0]
        )));
    }
    let yaml_file = PathBuf::from(&args[1]);
    let filename = &args[2];

    let banner: banner_builder::Banner = banner_builder::read_yaml_file(&yaml_file);
    let path = &std::path::Path::new(&filename).to_path_buf();
    let root = yaml_file.parent().unwrap();
    banner_builder::draw_image(&banner, root, path);

    Ok(())
}
