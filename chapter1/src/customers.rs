#[derive(Debug)]
pub struct Customer {
    id: String,
    name: String
}

impl Customer {
    pub fn new(id: String, name: String) -> Self {
        Self { id, name }
    }
}

#[derive(Debug)]
pub struct Customers {
    pub customers: Vec<Customer>
}

impl Customers {
    pub fn new(customers: Vec<Customer>) -> Self {
        Self { customers }
    }

    pub fn add(mut self, customer: Customer) -> Self {
        self.customers.push(customer);
        self
    }
}
