pub struct ShippingCost {
    base_price: u32,
}

impl ShippingCost {
    const MINIMUM_FOR_FREE: u32 = 3000;
    const COST:u32 = 500;

    pub fn new(base_price: u32) -> ShippingCost {
        ShippingCost {
            base_price
        }
    }

    pub fn amount(&self) -> u32 {
        if self.base_price < ShippingCost::MINIMUM_FOR_FREE {
            return ShippingCost::COST
        }

        self.base_price
    }
}