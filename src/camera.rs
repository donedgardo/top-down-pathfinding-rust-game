use bevy::app::{App, Plugin, Startup};
use bevy::prelude::*;

pub struct MyCameraPlugin;

impl Plugin for MyCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_main_camera);
    }
}

fn spawn_main_camera(mut commands: Commands) {
    commands.spawn((
        MainCamera,
        Camera3dBundle {
            transform: Transform::from_xyz(0., 40., -25.)
                .looking_at(Vec3::new(0., 0., 15.), Vec3::Y),
            ..default()
        },
    ));
}

#[derive(Component)]
pub struct MainCamera;

#[cfg(test)]
mod camera_test {
    use bevy::core_pipeline::core_2d::Core2dPlugin;
    use bevy::prelude::*;
    use crate::camera::{MainCamera, MyCameraPlugin};

    #[test]
    fn it_spawns_main_camera() {
        let mut app = App::new();
        app.add_plugins((Core2dPlugin, MyCameraPlugin));
        app.update();
        let result = app.world.query::<(&MainCamera, &Camera)>().get_single(&app.world);
        assert!(result.is_ok())
    }
}
