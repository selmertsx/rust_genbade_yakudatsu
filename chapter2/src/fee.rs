#[derive(Debug)]
pub struct Yen(u32);

pub trait Fee {
    fn fee(&self) -> Yen;
    fn label(&self) -> String;
}

pub struct AdultFee;
impl Fee for AdultFee {
    fn fee(&self) -> Yen {
        Yen(100)
    }

    fn label(&self) -> String {
        "大人".to_string()
    }
}

pub struct ChildFee;
impl Fee for ChildFee {
    fn fee(&self) -> Yen {
        Yen(50)
    }

    fn label(&self) -> String {
        "子供".to_string()
    }
}