use thiserror::Error;

#[derive(Error, Debug)]
pub enum QuantityError {
    #[error("range error")]
    RangeError,
}

#[derive(Debug)]
pub struct Quantity {
    value: u32
}

impl Quantity {
    const MIN:u32 = 1;
    const MAX:u32 = 100;

    pub fn new(value: u32) -> Result<Self, QuantityError> {
        match value {
            Quantity::MIN ..= Quantity::MAX => Ok(Quantity { value }),
            _ => Err(QuantityError::RangeError)
        }
    }

    pub fn add(self, other: Self) -> Result<Self, QuantityError> {
        let value = self.add_value(other);
        Self::new(value)
    }

    fn add_value(self, other: Self) -> u32 {
        return self.value + other.value;
    }
}
