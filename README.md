# Rust ts-scss-modules

This project is a small cli tool to create something in Rust
and learn the language. Handle CLI input, file traversal, parsing
and writing.

It is by no means a production ready project or targeted as such, 
such a learning project.


## Capabilities

This project can be used with:

``` shell
cargo run -- --path <path to your project> 
```

It will traverse all the files and folders scanning for .scss files
and render out a scss.d.ts file right next to it to be able to use
css modules within typescript.

## What it's not

A production ready optimized library to be used as part of your tool chain.
