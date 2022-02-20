use crate::fee::{Fee, Yen};

pub struct ReservationFees(Vec<Box<dyn Fee>>);

impl ReservationFees {
    pub fn push(&self, fee: Box<dyn Fee>) -> Self {
        // Self { self.fees }
        todo!()
    }
}

pub struct Reservation {
    fees: ReservationFees, 
}

impl Reservation {
    pub fn new(&self, fees: ReservationFees) -> Self {
        Self { fees }
    }

    pub fn addFees(&self, fee: Box<dyn Fee>) -> Self {
        self.fees.push(fee);
        Self { fees }
    }
}
