use std::sync::{Arc, RwLock};
use futures_lite::future;
use bevy::app::{App, Plugin, Update};
use oxidized_navigation::{NavMesh, NavMeshSettings, OxidizedNavigationPlugin};
use bevy_xpbd_3d::components::Collider;
use oxidized_navigation::debug_draw::{ DrawPath, OxidizedNavigationDebugDrawPlugin};
use bevy::prelude::*;
use bevy::math::Vec3;
use bevy::input::Input;
use bevy::tasks::{AsyncComputeTaskPool, Task};
use oxidized_navigation::query::find_path;
use oxidized_navigation::tiles::NavMeshTiles;

pub struct PathfindingPlugin {
    config: NavMeshSettings
}

impl PathfindingPlugin {
   pub fn default() -> Self {
       Self {
           config: NavMeshSettings {
               cell_width: 0.25,
               cell_height: 0.1,
               tile_width: 100,
               world_half_extents: 250.0,
               world_bottom_bound: -100.0,
               max_traversable_slope_radians: (40.0_f32 - 0.1).to_radians(),
               walkable_height: 20,
               walkable_radius: 1,
               step_height: 3,
               min_region_area: 100,
               merge_region_area: 500,
               max_contour_simplification_error: 1.1,
               max_edge_length: 80,
               max_tile_generation_tasks: Some(9),
           }
       }
   }
}

impl Plugin for PathfindingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            OxidizedNavigationPlugin::<Collider>::new(self.config.clone()),
            OxidizedNavigationDebugDrawPlugin,
        ))
            .insert_resource(AsyncPathfindingTasks::default())
            .add_systems(
                Update, (
                    run_async_pathfinding,
                    poll_pathfinding_tasks_system
                ));
    }
}

//  Async Pathfinding.
//  Press A to run.
//
//  Running pathfinding in a task without blocking the frame.
//  Also check out Bevy's async compute example.
//  https://github.com/bevyengine/bevy/blob/main/examples/async_tasks/async_compute.rs
//
// Holder resource for tasks.
#[derive(Default, Resource)]
struct AsyncPathfindingTasks {
    tasks: Vec<Task<Option<Vec<Vec3>>>>,
}

// Queue up pathfinding tasks.
fn run_async_pathfinding(
    keys: Res<Input<KeyCode>>,
    nav_mesh_settings: Res<NavMeshSettings>,
    nav_mesh: Res<NavMesh>,
    mut pathfinding_task: ResMut<AsyncPathfindingTasks>,
) {
    if !keys.just_pressed(KeyCode::A) {
        return;
    }
    let thread_pool = AsyncComputeTaskPool::get();
    let nav_mesh_lock = nav_mesh.get();
    let start_pos = Vec3::new(0.0, 1.0, 10.0);
    let end_pos = Vec3::new(-15.0, 1.0, -15.0);
    let task = thread_pool.spawn(async_path_find(
        nav_mesh_lock,
        nav_mesh_settings.clone(),
        start_pos,
        end_pos,
        None,
    ));
    pathfinding_task.tasks.push(task);
}

// Poll existing tasks.
fn poll_pathfinding_tasks_system(
    mut commands: Commands,
    mut pathfinding_task: ResMut<AsyncPathfindingTasks>,
) {
    // Go through and remove completed tasks.
    pathfinding_task.tasks.retain_mut(|task| {
        if let Some(string_path) = future::block_on(future::poll_once(task)).unwrap_or(None) {
            info!("Async path task finished with result: {:?}", string_path);
            commands.spawn(DrawPath {
                timer: Some(Timer::from_seconds(4.0, TimerMode::Once)),
                pulled_path: string_path,
                color: Color::BLUE,
            });
            false
        } else {
            true
        }
    });
}

/// Async wrapper function for path finding.
async fn async_path_find(
    nav_mesh_lock: Arc<RwLock<NavMeshTiles>>,
    nav_mesh_settings: NavMeshSettings,
    start_pos: Vec3,
    end_pos: Vec3,
    position_search_radius: Option<f32>,
) -> Option<Vec<Vec3>> {
    // Get the underlying nav_mesh.
    let Ok(nav_mesh) = nav_mesh_lock.read() else {
        return None;
    };
    // Run pathfinding to get a path.
    match find_path(
        &nav_mesh,
        &nav_mesh_settings,
        start_pos,
        end_pos,
        position_search_radius,
        Some(&[1.0, 0.5]),
    ) {
        Ok(path) => {
            info!("Found path (ASYNC): {:?}", path);
            return Some(path);
        }
        Err(error) => error!("Error with pathfinding: {:?}", error),
    }
    None
}

#[cfg(test)]
mod pathfinding_test {
    use bevy::input::InputPlugin;
    use bevy::prelude::*;
    use bevy_xpbd_3d::plugins::PhysicsPlugins;
    use crate::pathfinding::PathfindingPlugin;

    #[test]
    fn it_can_find_path_to_itself() {
        let mut app = App::new();
        app.add_plugins((
            MinimalPlugins,
            InputPlugin,
            TransformPlugin,
            PhysicsPlugins::default(),
            PathfindingPlugin::default()
        ));
        app.update();

    }
}


