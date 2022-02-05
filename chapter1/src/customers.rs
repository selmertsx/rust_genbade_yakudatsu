#[derive(Debug)]
pub struct Customer {
    pub id: String,
    pub name: String
}

#[derive(Debug)]
pub struct Customers {
    pub customers: Vec<Customer>
}

impl Customers {
    pub fn add(mut self, customer: Customer) -> Self {
        self.customers.push(customer);
        self
    }
}
