# Banner Builder


See the [Banner Builder](https://banner-builder.code-maven.com/) web site.

## Use as a library

TBD.

## Use as a command line tool:

### Install

```
cargo install banner-builder
```

Create a YAML file based on the example found on the [web site](https://banner-builder.code-maven.com/)


```
banbu hello_world.yaml hello_world.png
```


## Developmen

* Clone the repo
* Optionally set up pre-commit hook.


### Run tests:

```
cargo test
```

## Release and publish

* Update version number in Cargo.toml
* git commit
* git tag   (`git tag -a v0.2.2 -m "publish version v0.2.2"`)
* cargo publish


