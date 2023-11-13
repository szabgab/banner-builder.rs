use std::process::Command;

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn test_babu() {
        let (exit, stdout, stderr) = run("cargo run --bin babu hello_world.yaml hello_world.png");
        assert_eq!(exit, 1);
        assert_eq!(stdout, "");
        assert!(stderr
            .contains("Could not open file 'hello_world.yaml', error: No such file or directory"));

        let (exit, stdout, _stderr) =
            run("cargo run --bin babu examples/hello_world.yaml hello_world.png");
        assert_eq!(exit, 0);
        assert_eq!(stdout, "");
        //assert_eq!(stderr, "");
    }
}

fn run(command: &str) -> (i32, String, String) {
    let parts: Vec<&str> = command.split(' ').collect();

    let cmd = &parts[0];
    let args = &parts[1..parts.len()];

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
