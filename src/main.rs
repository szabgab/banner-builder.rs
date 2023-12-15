fn main() {
    simple_logger::init_with_env().unwrap();
    log::info!("Starting banner builder");

    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("{} config.yaml image.png", &args[0]);
        std::process::exit(1);
    }
    let yaml_file = &args[1];
    let filename = &args[2];

    let banner: banner_builder::Banner = banner_builder::read_yaml_file(yaml_file);
    let path = &std::path::Path::new(&filename).to_path_buf();
    banner_builder::draw_image(&banner, path);
}
