use bevy::prelude::*;
use bevy_mod_picking::DefaultPickingPlugins;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_xpbd_3d::prelude::PhysicsPlugins;
use pathfinding::PathfindingPlugin;
use crate::camera::MyCameraPlugin;
use crate::game_state::AppState;
use crate::gold_resource::ResourcesPlugin;
use crate::ui::UIPlugin;

use world::setup_3d_scene;
use crate::world::setup_scene;

mod gold_resource;
mod ui;
mod game_state;
mod camera;
mod supply;
mod pathfinding;
mod world;

fn main() {
    let mut app = App::new();
    app.add_state::<AppState>();
    app.add_plugins((
        DefaultPlugins,
        PhysicsPlugins::default(),
        PathfindingPlugin::default(),
        DefaultPickingPlugins,
        WorldInspectorPlugin::new(),
        MyCameraPlugin,
        UIPlugin,
        ResourcesPlugin,
    ));

    app.add_systems(Startup, setup_3d_scene);
    app.run();
}