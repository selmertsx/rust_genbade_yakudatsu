use crate::fee::{Fee, Yen};

pub struct ReservationFees(Vec<Box<dyn Fee>>);

impl ReservationFees {
    pub fn push(&mut self, fee: Box<dyn Fee>) {
        let fees = self.0;
        fees.push(fee)
    }
}

pub struct Reservation {
    fees: ReservationFees, 
}

impl Reservation {
    pub fn new(&self, fees: ReservationFees) -> Self {
        Self { fees }
    }

    pub fn addFees(&self, fee: Box<dyn Fee>) {
        self.fees.push(fee);
    }
}
