---
title: Use the Banner Builder as a library
timestamp: 2024-05-16T17:30:01
description: Generating images from inside Rust code
published: true
---

## Use as a library

```
cargo add banner-builder
```


```rust
fn main() {
        let name = "hello_world";
        let filename = "test.png";
        let banner = banner_builder::Banner {
            width: 1000,
            height: 500,
            text: "Hello World!".to_owned(),
            background_color: "F0F0FF".to_owned(),
        };
        let path = &std::path::Path::new(&filename).to_path_buf();
        banner_builder::draw_image(&banner, path);
}
```

