#[derive(Debug)]
pub struct Yen(u32);

pub trait Fee {
    fn fee() -> Yen;
    fn label() -> String;
}

pub struct AdultFee;
impl Fee for AdultFee {
    fn fee() -> Yen {
        Yen(100)
    }

    fn label() -> String {
        "大人".to_string()
    }
}

pub struct ChildFee;
impl Fee for ChildFee {
    fn fee() -> Yen {
        Yen(50)
    }

    fn label() -> String {
        "子供".to_string()
    }
}