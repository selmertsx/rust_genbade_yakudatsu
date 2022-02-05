mod fee;
use fee::ChildFee;

fn main() {
    let fee = ChildFee::fee();
    println!("{:?}", fee);
}
