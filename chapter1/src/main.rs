mod shipping_cost;
use shipping_cost::ShippingCost;

mod quantity;
use quantity::Quantity;

fn main() {
    println!("====== Shipping Cost ======");
    let shipping_cost = ShippingCost::new(2900);
    println!("{}", shipping_cost.amount());
    let shipping_cost = ShippingCost::new(3100);
    println!("{}", shipping_cost.amount());

    println!("====== Quantity ======");
    let quantity = Quantity::new(32).unwrap();
    println!("quantity: {:?}", quantity);
    let other = Quantity::new(20).unwrap();
    let added = quantity.add(other).unwrap();
    println!("added: {:?}", added);
    // 借用よくわかってない。もっと良いやり方がある気がする
    let quantity = Quantity::new(32).unwrap();
    let over = Quantity::new(70).unwrap();
    match quantity.add(over) {
        Ok(val) => println!("over {:?}", val),
        Err(err) => println!("{}", err),
    }
}
