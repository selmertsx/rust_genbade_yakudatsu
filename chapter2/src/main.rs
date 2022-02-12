mod fee;
use fee::{ ChildFee, Fee };

mod reservation;
use reservation::Reservation;

fn main() {
    let fee = ChildFee;
    let fee = fee.fee();
    println!("{:?}", fee);
}
