# Files for the Rust Programing Language book

This is my code as I go through the online book,  
"The Rust Programming Language"  
https://doc.rust-lang.org/book/  

It's broken down by chapter numbers, which might change since the online book may get updated at any point.  

## Supporting a single VSCode workspace containing multiple rust binaries

It's nicer to just have a single VSCode window open at the root of the repo, AKA a single VSCode workspace that contains all the rust binary projects. The root of the repo contains a `Cargo.toml`, which defines a [Cargo Workspace](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html) and references each project. If we don't have this, the `rust-analyzer` freaks out and can't find the projects.  
