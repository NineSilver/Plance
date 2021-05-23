# Plance
> *Plain Plance*

Plance is a [Cargo-like](https://doc.rust-lang.org/cargo/) project manager for C and C++ written in Rust.
Currently is in an early-stage, so I hope it has 0 bugs.

## Features

- [X] Successfully creates directories
- [X] Init command
- [X] Git integration (if it is present, it'll be executed in the given directory)
- [] Config files
- [] Build system

## Building

Simply do:
`cargo build --release`

EZ!

## Working with Plance

Plance allows you to manage projects very easily:

- To create a new project: `plance new <name> [-t type]`, where *type* can be C", C++ or default
- To initialize a new project in an existing directory: `plance init [--name name] [-t type] [--create-source]`, where *name* is the name of the project (defaults to the directory name), and *create-source* instructs Plance to also create the source directory using the given project type
- To delete a project use `plance delete <dirname>`, where *dirname* is the location of the project.

## Contributing

Any contribution to Plance project is appreciated!
