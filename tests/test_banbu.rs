use std::process::Command;

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn test_with_struct() {
        let name = "hello_world";
        let filename = "test.png";
        let banner = banner_builder::Banner {
            width: 1000,
            height: 500,
            text: "Hello World!".to_owned(),
            background_color: "FFFFFF".to_owned(),
        };
        let path = &std::path::Path::new(&filename).to_path_buf();
        banner_builder::draw_image(&banner, path);

        let (exit, stdout, stderr) =
            run(format!("diff site/examples/{}.png test.png", name).as_str());
        assert_eq!(exit, 0);
        assert_eq!(stdout, "");
        assert_eq!(stderr, "");
    }

    #[test]
    fn test_banbu_missing() {
        std::env::set_var("RUST_LOG", "warn");

        let (exit, stdout, stderr) = run("cargo run --bin banbu hello_world.yaml hello_world.png");
        assert_eq!(exit, 1);
        assert_eq!(stdout, "");
        assert!(stderr
            .contains("Could not open file 'hello_world.yaml', error: No such file or directory"));
    }

    #[test]
    fn test_banbu() {
        for name in [
            "hello_world",
            "youtube_thumbnail_text_background",
            "wrap_text",
        ] {
            let cmd = format!("target/debug/banbu site/examples/{}.yaml test.png", name);
            println!("{}", cmd);
            let (exit, stdout, stderr) = run(&cmd);
            assert_eq!(stderr, "");
            assert_eq!(stdout, "");
            assert_eq!(exit, 0);

            let (exit, stdout, stderr) =
                run(format!("diff site/examples/{}.png test.png", name).as_str());
            assert_eq!(exit, 0);
            assert_eq!(stdout, "");
            assert_eq!(stderr, "");
        }
    }

    #[test]
    fn test_banner_builder() {
        for name in [
            "hello_world",
            "youtube_thumbnail_text_background",
            "wrap_text",
        ] {
            let filename = "test.png";
            let yaml_file = format!("site/examples/{}.yaml", name);
            let banner: banner_builder::Banner = banner_builder::read_yaml_file(&yaml_file);
            let path = &std::path::Path::new(&filename).to_path_buf();
            banner_builder::draw_image(&banner, path);

            let (exit, stdout, stderr) =
                run(format!("diff site/examples/{}.png test.png", name).as_str());
            assert_eq!(exit, 0);
            assert_eq!(stdout, "");
            assert_eq!(stderr, "");
        }
    }
}

fn run(command: &str) -> (i32, String, String) {
    let parts: Vec<&str> = command.split(' ').collect();

    let cmd = &parts[0];
    let args = &parts[1..parts.len()];
    println!("cmd:  '{}'", cmd);
    println!("args: '{:?}'", args);

    let result = Command::new(cmd)
        .args(args)
        .output()
        .expect("ls command failed to start");

    (
        result.status.code().unwrap(),
        std::str::from_utf8(&result.stdout).unwrap().to_owned(),
        std::str::from_utf8(&result.stderr).unwrap().to_owned(),
    )
}
