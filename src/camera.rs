use bevy::app::{App, Plugin, Startup};
use bevy::prelude::{Camera2dBundle, Commands, Component};

pub struct MyCameraPlugin;

impl Plugin for MyCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_main_camera);
    }
}

fn spawn_main_camera(mut commands: Commands) {
    commands.spawn((MainCamera, Camera2dBundle::default()));

}

#[derive(Component)]
pub struct MainCamera;

#[cfg(test)]
mod camera_test {
    use bevy::core_pipeline::core_2d::Core2dPlugin;
    use bevy::prelude::*;
    use crate::camera::MainCamera;
    use crate::camera::MyCameraPlugin;

    #[test]
    fn it_spawns_main_camera() {
        let mut app =  App::new();
        app.add_plugins((Core2dPlugin, MyCameraPlugin));
        app.update();
        let result = app.world.query::<(&MainCamera, &Camera)>().get_single(&app.world);
        assert!(result.is_ok())

    }

}
