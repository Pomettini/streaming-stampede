extern crate ggez;

use ggez::*;
use std::{env, path};

// Using custom version of event
pub mod assets;
pub mod constants;
pub mod player;
pub mod pokemon_types;
pub mod pokemons;
pub mod states;
pub mod ui;
pub mod utils;
pub mod pokemon_sprite;

use constants::*;
use states::*;

pub fn main() {
    let mut cb = ContextBuilder::new("Streaming Stampede", "Pomettini")
        .window_setup(conf::WindowSetup::default().title("Streaming Stampede"))
        .window_mode(
            conf::WindowMode::default().dimensions(WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32),
        );

    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources/");
        cb = cb.add_resource_path(path);
    }

    let ctx = &mut cb.build().unwrap();

    let state = &mut MainState::new(ctx).unwrap();

    event::run(ctx, state).unwrap();
}
