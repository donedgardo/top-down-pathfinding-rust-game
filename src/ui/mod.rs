use bevy::app::{App, Plugin};
use bevy::prelude::{Changed, Commands, Component, OnEnter, Query, Text, TextBundle, TextStyle, With};
use crate::game_state::AppState;
use crate::gold_resource::GoldResource;

pub fn setup_gold_resource_ui(mut commands: Commands) {
    commands.spawn((
        GoldResourceLabel,
        TextBundle::from_section(
            "0".to_string(),
            TextStyle::default(),
        ))
    );
}

pub fn update_gold_resource_label(query: Query<&GoldResource, Changed<GoldResource>>, mut text_query: Query<&mut Text, With<GoldResourceLabel>>) {
    let mut text = text_query.single_mut();
    for gold_resource in query.iter() {
        text.sections[0].value = gold_resource.balance().to_string();
    }
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
    use crate::gold_resource::GoldResource;
    use crate::ui::{GoldResourceLabel, UIPlugin, update_gold_resource_label};

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

    #[test]
    fn it_updates_label_when_resource_changes() {
        //setup
        let mut app = setup();
        let entity = app.world.spawn(GoldResource::new(0)).id();
        app.update();

        // action
        let mut binding = app.world.entity_mut(entity);
        let mut gold_resource = binding.get_mut::<GoldResource>().unwrap();
        gold_resource.gain(50);
        app.update();

        //result
        let text = app.world.query_filtered::<&Text, With<GoldResourceLabel>>()
            .single(&app.world);
        assert_eq!(text.sections[0].value, "50");
    }

    fn setup() -> App {
        let mut app = App::new();
        app.add_plugins((Core2dPlugin, UIPlugin));
        app.add_state::<AppState>();
        app.add_systems(
            Update,
            update_gold_resource_label.run_if(in_state(AppState::InGame)));
        app.update();
        app
    }
}
