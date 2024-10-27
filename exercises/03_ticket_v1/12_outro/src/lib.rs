// TODO: Define a new `Order` type.
pub struct Order {
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
    product_name: String,
    quantity: i32,
    unit_price: i32
}
impl Order {
    pub fn new(name: String, quantity: i32, unit_price: i32) -> Order {
        //   The product name can't be empty and it can't be longer than 300 bytes.
        if name.is_empty() || name.len() > 300 {
            panic!("The product name can't be empty and it can't be longer than 300 bytes.")
        }
        if quantity < 1 {
        //   The quantity must be strictly greater than zero.
            panic!("The quantity must be strictly greater than zero.")
        }
        if unit_price < 1 {
        //   The unit price is in cents and must be strictly greater than zero.
            panic!("Order must include a method named `total` that returns the total price of the order.")
        }
        Order {
            product_name: name,
            quantity: quantity,
            unit_price: unit_price
        }
    }
    pub fn product_name(&self) -> &String {
        &self.product_name
    }
    pub fn quantity(&self) -> &i32 {
        &self.quantity
    }
    pub fn unit_price(&self) -> &i32 {
        &self.unit_price
    }
    pub fn total(&self) -> i32{
    //   Order must include a method named `total` that returns the total price of the order.
        return &self.quantity * &self.unit_price;
    }

    //   Order must provide setters and getters for each field.
    pub fn set_product_name(&mut self, name: String) ->() {
        if name.is_empty() || name.len() > 300 {
            panic!("The product name can't be empty and it can't be longer than 300 bytes.")
        }
        self.product_name = name
    }
    pub fn set_quantity(&mut self, quantity: i32) ->() {
        if quantity < 1 {
        //   The quantity must be strictly greater than zero.
            panic!("The quantity must be strictly greater than zero.")
        }
        self.quantity = quantity
    }
    pub fn set_unit_price (&mut self, unit_price: i32) ->() {
        if unit_price < 1 {
        //   The unit price is in cents and must be strictly greater than zero.
            panic!("Order must include a method named `total` that returns the total price of the order.")
        }
        self.unit_price =  unit_price
    }
 
}


// let mut order: Order = Order::new("eggs", 23, 2)
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.
