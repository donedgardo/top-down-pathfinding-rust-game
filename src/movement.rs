use bevy::math::Vec3;
use bevy::prelude::*;
use bevy_xpbd_3d::prelude::{LinearVelocity, Position};
use crate::game_state::AppState;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<MovementPath>();
        app.add_systems(Update, movement_system.run_if(in_state(AppState::InGame)));
    }
}

#[derive(Component, Default, Reflect)]
pub struct MovementPath(Vec<Vec3>);

impl MovementPath {
    pub fn new(path: Vec<Vec3>) -> Self {
        Self(path)
    }
    pub fn remove_first(&mut self){
        if !self.0.is_empty() {
            self.0.remove(0);
        }
    }
}

fn movement_system(mut q: Query<(&mut MovementPath, &mut Transform, &mut LinearVelocity)>, timer: Res<Time>) {
    for (mut path, mut transform, mut velocity) in q.iter_mut() {
        if !path.0.is_empty() {
            if (transform.translation - path.0[0]).length() <= 1.2 {
                info!("Reach destination");
                velocity.0 = Vec3::ZERO;
                path.remove_first();
            } else {
                let mut direction = (path.0[0] - transform.translation).normalize(); // Get the first direction
                direction.y = 0.;
                let speed = 10.;
                velocity.0 = direction * speed;
                // Calculate the movement based on speed and time
                //transform.translation += direction * speed * timer.delta_seconds();
            }
        }
    }

}
