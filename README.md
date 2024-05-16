# Banner Builder


See the [Banner Builder](https://banner-builder.code-maven.com/) web site.


## Development

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

* Update version number in Cargo.toml
* Update the CHANGES.md file
* run `cargo fmt`
* run `cargo clippy`
* run `cargo test`
* `git commit`
* `cargo publish`
* git tag using the same version number:   (`git tag -a v0.2.2 -m "publish version v0.2.2"`)
* `git push --tags`


