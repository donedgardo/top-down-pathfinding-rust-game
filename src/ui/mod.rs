use bevy::app::{App, Plugin};
use bevy::prelude::{Commands, Component, OnEnter, TextBundle, TextStyle};
use crate::game_state::AppState;

pub fn setup_gold_resource_ui(mut commands: Commands) {
    commands.spawn((
        GoldResourceLabel,
        TextBundle::from_section(
            "0".to_string(),
            TextStyle::default(),
        ))
    );
}



pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), setup_gold_resource_ui);
    }
}

#[derive(Component)]
pub struct GoldResourceLabel;

#[cfg(test)]
mod resources_ui_test {
    use bevy::core_pipeline::core_2d::Core2dPlugin;
    use bevy::prelude::*;
    use crate::game_state::AppState;
    use crate::ui::{GoldResourceLabel, UIPlugin};

    #[test]
    fn it_shows_gold_resources_label() {
        let mut app = setup();
        assert_eq!(app.world.query::<&GoldResourceLabel>()
                       .iter(&app.world).len(), 1);
    }

    #[test]
    fn it_shows_gold_resources_label_default_0() {
        let mut app = setup();
        let text = app.world.query_filtered::<&Text, With<GoldResourceLabel>>()
            .single(&app.world);
        assert_eq!(text.sections[0].value, "0");
    }

    fn setup() -> App {
        let mut app = App::new();
        app.add_plugins((Core2dPlugin, UIPlugin));
        app.add_state::<AppState>();
        app.update();
        app
    }
}
