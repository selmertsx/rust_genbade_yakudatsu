pub struct ShippingCost {
    base_price: u32,
}

impl ShippingCost {
    const MINIMUM_FOR_FREE: u32 = 3000;
    pub const COST:u32 = 500;
    pub const FREE: u32 = 0;

    pub fn new(base_price: u32) -> Self {
        ShippingCost {
            base_price
        }
    }

    // TODO: 
    // 返す型の部分で ShippingCost::FREE | ShippingCost::COST のどちらかが必ず帰る。という実装をしたい
    pub fn amount(&self) ->  u32 {
        if self.base_price < ShippingCost::MINIMUM_FOR_FREE {
            return ShippingCost::COST
        }

        ShippingCost::FREE
    }
}
