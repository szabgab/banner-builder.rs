
fn main() {
    println!("Hello, world!");
    let filename = "hello.png".to_string();
    let path = &std::path::Path::new(&filename).to_path_buf();
    banner_builder::draw_image(path, "Hello World");
}
