use bevy::prelude::*;

struct GoldResource(u32);
#[derive(Debug, Clone, PartialEq)]
struct NotEnoughResourceError;

impl GoldResource {
    pub fn gain(&mut self, amount: u32) {
        self.0 += amount;
    }

    pub(crate) fn remove(&mut self, amount: u32) -> Result<(), NotEnoughResourceError> {
        if amount > self.0 {
            return Err(NotEnoughResourceError);
        }
        self.0 -= amount;
        Ok(())
    }
    pub fn balance(&self) -> u32 {
        self.0
    }
}

#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default)]
enum AppState {
    #[default]
    InGame,
}

fn setup_in_game_ui(mut commands: Commands) {
    commands.spawn((
        GoldResourceLabel,
        TextBundle::from_section(
            "0".to_string(),
            TextStyle::default(),
        ))
    );
}

fn main() {
    println!("Hello, world!");
}

#[derive(Component)]
pub struct GoldResourceLabel;

#[cfg(test)]
mod resources_test {
    use bevy::core_pipeline::core_2d::Core2dPlugin;
    use bevy::prelude::*;
    use crate::*;

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
    fn it_adds_gold_resource() {
        let mut gold_resource = GoldResource(0);
        gold_resource.gain(5);
        assert_eq!(gold_resource.balance(), 5);
        gold_resource.gain(6);
        assert_eq!(gold_resource.balance(), 11);
    }

    #[test]
    fn it_remains_the_same_when_removing_0() {
        let mut gold_resource = GoldResource(0);
        gold_resource.remove(0).unwrap();
        assert_eq!(gold_resource.balance(), 0);
    }

    #[test]
    fn it_errors_if_not_enough_balance_when_removing() {
        let mut gold_resource = GoldResource(0);
        let result = gold_resource.remove(5);
        assert_eq!(result.unwrap_err(), NotEnoughResourceError);
    }
    #[test]
    fn it_removes_correctly_from_balance() {
        let mut gold_resource = GoldResource(4);
        let result = gold_resource.remove(3);
        assert_eq!(gold_resource.balance(), 1);
    }

    fn setup() -> App {
        let mut app = App::new();
        app.add_plugins(Core2dPlugin);
        app.add_systems(OnEnter(AppState::InGame), setup_in_game_ui);
        app.add_state::<AppState>();
        app.update();
        app
    }
}
