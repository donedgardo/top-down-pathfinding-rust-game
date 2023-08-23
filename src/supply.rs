#[derive(Debug, PartialEq)]
pub struct NotEnoughSupplyError;

impl Supply {
    pub fn amount(&self) -> u32 {
        self.0
    }

    pub fn capacity(&self) -> u32 {
        self.1
    }

    pub fn add_amount(&mut self, x: u32) -> Result<(), NotEnoughSupplyError> {
        if  self.0 + x > self.1 {
            return Err(NotEnoughSupplyError);
        }
        self.0 += x;
        Ok(())
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
    use crate::supply::Supply;
    use crate::supply::NotEnoughSupplyError;

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

    #[test]
    fn it_can_add_amount_to_supply() {
        let mut supply = Supply::default();
        supply.add_capacity(1);
        let _ = supply.add_amount(1);
        assert_eq!(supply.amount(), 1);
    }

    #[test]
    fn it_cant_add_amount_without_enough_capacity() {
        let mut supply = Supply::default();
        let result = supply.add_amount(1);
        assert_eq!(result.unwrap_err(), NotEnoughSupplyError);
    }
}

#[derive(Default)]
pub struct Supply(u32, u32);
