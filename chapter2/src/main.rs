mod fee;
use fee::{ ChildFee, Fee };

fn main() {
    let fee = ChildFee::fee();
    println!("{:?}", fee);
}
