
# Dawn

Note this code requires the Dusk library
https://github.com/FrazorBladezSharp/dusk



Ideas:

lets try a simple Game Development Sandbox
    we can use  SFML - for windows graphics and keyboard input
                SPECS - for a simple ecs setup
                SPECS-DERIVE - for macros ie #[derive(Component)]
                EGUI - for an Easy Graphical User Interface
                EGUI-SFML - for integration

****************************************************************************************

Rust Book
https://doc.rust-lang.org/book/
https://doc.rust-lang.org/std/index.html
$ cargo doc --open

sfml        for window & sprites
https://docs.rs/sfml/latest/sfml/
https://github.com/jeremyletang/rust-sfml

sfml learning reference guide
https://www.sfml-dev.org/tutorials/3.0/graphics/draw/#introduction

specs + specs-derive ecs
https://docs.rs/specs/latest/specs/
    rogue tutorial using rltk
https://bfnightly.bracketproductions.com/rustbook/chapter_0.html
https://bfnightly.bracketproductions.com/chapter_2.html

egui-sfml   for GUI
https://github.com/crumblingstatue/egui-sfml
https://github.com/emilk/egui

***************************************************************************************

Setting up a workspace eg :
    create a directory (eg midnight) in which we can create multiple sub directories
    First sub directory for a lib
    Second sub directory for an app

    In the main directory place Cargo.toml containing eg:
    
    [workspace]

    resolver = "2"

    members = [
        "dusk",     // our lib
        "dawn",     // our App
    ]

    in the sub directory for the app, Cargo.toml eg:

    [package]
    name = "dawn"
    version = "0.1.0"
    edition = "2021"

    [dependencies]
    dusk = { path = "../dusk"}

we can then build from the main directory which adds our lib to the app

***********************************************************************************
