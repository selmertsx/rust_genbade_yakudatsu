use crate::fee::Fee;

pub struct Reservation {
    fees: Vec<Fee>, 
}

impl Reservation {
    pub fn new(&self, fees: Vec<Fee>) -> Self {
        Self { fees }
    }
}
