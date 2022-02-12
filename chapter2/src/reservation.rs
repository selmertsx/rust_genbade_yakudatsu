use crate::fee::Fee;

pub struct Reservation {
    fees: Vec<Box<dyn Fee>>, 
}

impl Reservation {
    pub fn new(&self, fees: Vec<Box<dyn Fee>>) -> Self {
        Self { fees }
    }
}
