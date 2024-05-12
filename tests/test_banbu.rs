use std::process::Command;

#[cfg(test)]
mod tests {
    use super::run;
    use std::path::PathBuf;

    #[test]
    fn test_with_struct() {
        //std::env::set_var("RUST_LOG", "warn");

        let name = "hello_world";
        let banner = banner_builder::Banner {
            width: 1000,
            height: 500,
            text: "Hello World!".to_owned(),
            background_color: "FFFFFF".to_owned(),
            embed: vec![],
        };

        let tmp_dir = tempfile::tempdir().unwrap();
        let path = tmp_dir.path().join("test.png");
        banner_builder::draw_image(&banner, &PathBuf::new(), &path);

        let (exit, stdout, stderr) =
            run(format!("diff site/examples/{}.png {}", name, path.display()).as_str());
        assert_eq!(exit, 0);
        assert_eq!(stdout, "");
        assert_eq!(stderr, "");
    }

    #[test]
    fn test_banbu_missing() {
        //std::env::set_var("RUST_LOG", "warn");

        let (exit, stdout, stderr) = run("cargo run --bin banbu hello_world.yaml hello_world.png");
        assert_eq!(exit, 1);
        assert_eq!(stdout, "");
        //assert_eq!(stderr, "");
        assert!(stderr.contains("Could not open file"));
        assert!(stderr.contains("hello_world.yaml"));
        assert!(stderr.contains("error: No such file or directory"));
    }

    #[test]
    fn test_banbu() {
        std::env::set_var("RUST_LOG", "warn");

        for name in [
            "hello_world",
            "youtube_thumbnail_text_background",
            "wrap_text",
            "embed_image",
        ] {
            let tmp_dir = tempfile::tempdir().unwrap();
            let path = tmp_dir.path().join("test.png");

            let cmd = format!(
                "target/debug/banbu site/examples/{}.yaml {}",
                name,
                path.display()
            );
            println!("test_banbu cmd: {}", cmd);
            let (exit, stdout, stderr) = run(&cmd);
            assert_eq!(stderr, "");
            assert_eq!(stdout, "");
            assert_eq!(exit, 0);

            let (exit, stdout, stderr) =
                run(format!("diff site/examples/{}.png {}", name, path.display()).as_str());
            assert_eq!(stdout, "");
            assert_eq!(stderr, "");
            assert_eq!(exit, 0);
        }
    }

    #[test]
    fn test_banner_builder() {
        std::env::set_var("RUST_LOG", "warn");

        for name in [
            "hello_world",
            "youtube_thumbnail_text_background",
            "wrap_text",
            "embed_image",
        ] {
            let yaml_file = PathBuf::from(format!("site/examples/{name}.yaml"));
            let banner: banner_builder::Banner = banner_builder::read_yaml_file(&yaml_file);

            let tmp_dir = tempfile::tempdir().unwrap();
            let path = tmp_dir.path().join("test.png");

            banner_builder::draw_image(&banner, &PathBuf::from("site/examples"), &path);

            let (exit, stdout, stderr) =
                run(format!("diff site/examples/{}.png {}", name, path.display()).as_str());
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
