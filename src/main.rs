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

#[derive(Default)]
pub struct Supply(u32, u32);

impl Supply {
    pub fn amount(&self) -> u32 {
        self.0
    }

    pub fn capacity(&self) -> u32 {
        self.1
    }

    pub fn add_capacity(&mut self, x: u32) {
        self.1 += x;
    }

    pub fn remove_capacity(&mut self, x: u32) {
        if x > self.1 {
            self.1 = 0;

        } else {
            self.1 -= x;
        }
    }
}

#[cfg(test)]
mod supply_tests {
    use crate::{Supply};

    #[test]
    fn it_starts_empty_capacity() {
        let supply = Supply::default();
        assert_eq!(supply.capacity(), 0);
    }

    #[test]
    fn it_starts_empty_amount() {
        let supply = Supply::default();
        assert_eq!(supply.amount(), 0);
    }

    #[test]
    fn it_can_add_to_capacity() {
        let mut supply = Supply::default();
        supply.add_capacity(10);
        assert_eq!(supply.capacity(), 10);
    }

    #[test]
    fn it_can_remove_to_capacity() {
        let mut supply = Supply::default();
        supply.add_capacity(10);
        supply.remove_capacity(10);
        assert_eq!(supply.capacity(), 0);
    }

    #[test]
    fn it_cant_remove_more_capacity_than_it_has() {
        let mut supply = Supply::default();
        supply.remove_capacity(1);
        assert_eq!(supply.capacity(), 0);

    }
}
