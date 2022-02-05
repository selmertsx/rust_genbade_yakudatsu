mod shipping_cost;
use shipping_cost::ShippingCost;

mod quantity;
use quantity::Quantity;

mod customers;
use customers::{ Customers, Customer };

fn main() {
    println!("====== Shipping Cost ======");
    let shipping_cost = ShippingCost::new(2900);
    println!("{}", shipping_cost.amount());
    let shipping_cost = ShippingCost::new(3100);
    println!("{}", shipping_cost.amount());

    // unwrapや借用の面で
    // もっと良いやり方がある気がする
    // tryとか使えばええんかな
    println!("====== Value Object: Quantity ======");
    let quantity = Quantity::new(32).unwrap();
    println!("quantity: {:?}", quantity);
    let other = Quantity::new(20).unwrap();
    let added = quantity.add(other).unwrap();
    println!("added: {:?}", added);
    let quantity = Quantity::new(32).unwrap();
    let over = Quantity::new(70).unwrap();
    match quantity.add(over) {
        Ok(val) => println!("over {:?}", val),
        Err(err) => println!("{}", err),
    }

    println!("====== Collection Object: Customers =======");
    let customer_a = Customer::new("xxx1".to_string(), "Mr. A".to_string());
    let customer_b = Customer::new("xxx2".to_string(), "Mr. B".to_string());
    let customers = Customers::new(vec![customer_a]);
    println!("{:?}", customers);
    let added_customers = customers.add(customer_b);
    println!("{:?}", added_customers);
}
