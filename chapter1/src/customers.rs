use std::fmt;
use std::fmt::{Display};

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

impl Display for Customer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Customer id: {}, name: {}", self.id, self.name)
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

        Self { customers: self.customers }
    }
}
