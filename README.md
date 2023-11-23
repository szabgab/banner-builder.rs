# Banner Builder


See the [Banner Builder](https://banner-builder.code-maven.com/) web site.

## Use as a library

TBD.

## Use as a command line tool:

### Install as a CLI tool

If you have Rust on your system you can type in the following command:

```
cargo install banner-builder
```

If you don't have Rust, we'll supply a pre-compiled binary. TODO

### Use as a CLI tool:

Create a YAML file based on the example found on the [web site](https://banner-builder.code-maven.com/)

Run the command supplying the path to the YAML file and the path to the image you'd like to generate:

```
banbu hello_world.yaml hello_world.png
```


## Development

* Clone the repo: `git clone https://github.com/szabgab/banner-builder.rs/`
* Optionally set up pre-commit hook:
    * Install [pre-commit](https://pre-commit.com/)
    * In the cloned repository run `pre-commit install`


### Run tests:

```
cargo test
```

## Release and publish

* Update version number in Cargo.toml
* `git commit`
* `cargo publish`
* git tag using the same version number:   (`git tag -a v0.2.2 -m "publish version v0.2.2"`)
* `git push --tags`


