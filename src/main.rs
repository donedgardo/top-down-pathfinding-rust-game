use bevy::prelude::*;

mod gold_resource;
mod ui;
mod game_state;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.run();
}
