use bevy::prelude::*;
use crate::gold_resource::ResourcesPlugin;
use crate::ui::UIPlugin;

mod gold_resource;
mod ui;
mod game_state;

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins,
        UIPlugin,
        ResourcesPlugin,
    ));
    app.run();
}
