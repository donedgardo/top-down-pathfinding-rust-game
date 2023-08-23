#[derive(Default)]
pub struct GoldResource(u32);

impl GoldResource {
    pub fn new(amount: u32) -> Self {
        Self(amount)
    }
    pub fn gain(&mut self, amount: u32) {
        self.0 += amount;
    }

    pub fn remove(&mut self, amount: u32) -> Result<(), NotEnoughResourceError> {
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

#[derive(Debug, Clone, PartialEq)]
pub struct NotEnoughResourceError;

mod resources_ui_test {
    use crate::gold_resource::{GoldResource, NotEnoughResourceError};

    #[test]
    fn it_adds_gold_resource() {
        let mut gold_resource = GoldResource::default();
        gold_resource.gain(5);
        assert_eq!(gold_resource.balance(), 5);
        gold_resource.gain(6);
        assert_eq!(gold_resource.balance(), 11);
    }

    #[test]
    fn it_remains_the_same_when_removing_0() {
        let mut gold_resource = GoldResource::default();
        gold_resource.remove(0).unwrap();
        assert_eq!(gold_resource.balance(), 0);
    }

    #[test]
    fn it_errors_if_not_enough_balance_when_removing() {
        let mut gold_resource = GoldResource::default();
        let result = gold_resource.remove(5);
        assert_eq!(result.unwrap_err(), NotEnoughResourceError);
    }
    #[test]
    fn it_removes_correctly_from_balance() {
        let mut gold_resource = GoldResource::new(4);
        let result = gold_resource.remove(3);
        assert_eq!(gold_resource.balance(), 1);
    }
}