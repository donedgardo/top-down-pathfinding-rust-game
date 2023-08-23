use bevy::prelude::*;
use crate::camera::MyCameraPlugin;
use crate::game_state::AppState;
use crate::gold_resource::ResourcesPlugin;
use crate::ui::UIPlugin;

mod gold_resource;
mod ui;
mod game_state;
mod camera;

fn main() {
    let mut app = App::new();
    app.add_state::<AppState>();
    app.add_plugins((
        DefaultPlugins,
        MyCameraPlugin,
        UIPlugin,
        ResourcesPlugin,
    ));
    app.run();
}
