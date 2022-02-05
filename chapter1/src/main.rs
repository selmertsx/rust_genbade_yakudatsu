mod shipping_cost;
use shipping_cost::ShippingCost;

fn main() {
    let shipping_cost = ShippingCost::new(2900);
    println!("{}", shipping_cost.amount());
    
    let shipping_cost = ShippingCost::new(3100);
    println!("{}", shipping_cost.amount());
}
