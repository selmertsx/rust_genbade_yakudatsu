pub struct ShippingCost {
    base_price: u32,
}

impl ShippingCost {
    pub fn new(base_price: u32) -> ShippingCost {
        ShippingCost {
            base_price
        }
    }

    pub fn amount(&self) -> u32 {
        if self.base_price < 3000 {
            return 500
        }

        self.base_price
    }
}