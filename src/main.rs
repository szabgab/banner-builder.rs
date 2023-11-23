fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("{} config.yaml image.png", &args[0]);
        std::process::exit(1);
    }
    let yaml_file = &args[1];
    let banner = read_yaml_file(yaml_file);
    let filename = &args[2];

    let path = &std::path::Path::new(&filename).to_path_buf();
    banner_builder::draw_image(&banner, path);
}

fn read_yaml_file(yaml_file: &String) -> banner_builder::Banner {
    let banner: banner_builder::Banner = match std::fs::File::open(yaml_file) {
        Ok(file) => match serde_yaml::from_reader(file) {
            Ok(content) => content,
            Err(error) => {
                eprintln!("Error parsing '{yaml_file}', error: {error}");
                std::process::exit(1);
            }
        },
        Err(error) => {
            eprintln!("Could not open file '{yaml_file}', error: {error}");
            std::process::exit(1);
        }
    };
    banner
}
