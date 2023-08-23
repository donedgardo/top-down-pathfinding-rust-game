use bevy::prelude::*;

mod gold_resource;
mod ui;

#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default)]
enum AppState {
    #[default]
    InGame,
}

fn main() {
    println!("Hello, world!");
}
