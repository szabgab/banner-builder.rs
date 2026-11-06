# Development

* Clone the repo: `git clone https://github.com/szabgab/banner-builder.rs/`
* Optionally set up pre-commit hook:
    * Install [pre-commit](https://pre-commit.com/)
    * In the cloned repository run `pre-commit install`


### Run tests:

```
cargo test
```

## Regenerate example images

```
./regenerate_images.sh
```

## Release and publish

* Update version number in this file and in Cargo.toml to 0.2.9
* Update the CHANGES.md file
* `cargo fmt`
* `cargo clippy`
* `cargo test`
* `git add .`
* `git commit -m "prepare for 0.2.9"`
* `git push`
* `cargo publish`
* `git tag -a v0.2.9 -m "publish version v0.2.9"
* `git push --tags`


