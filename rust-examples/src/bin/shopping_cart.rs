/*Shopping Cart
Create Product and ShoppingCart structs.
Add/remove products and calculate the total. */

struct Product {
    name : String,
    price : f64
}

struct ShoppingCart {
    products: Vec<Product>,
}

impl ShoppingCart {
    fn new() -> ShoppingCart {
        ShoppingCart { products: Vec::new() }
    }

    fn add_product(&mut self, product: Product) {
        self.products.push(product);
    }

    fn remove_product(&mut self, name: &str) {
        self.products.retain(|product| product.name != name);
    }

    fn total(&self) -> f64 {
        self.products.iter().fold(0.0, |total, product| total + product.price)
    }
}

fn main () {
    let product1 = Product {
        name : String::from("Laptop"),
        price : 1000.0
    };

    let product2 = Product {
        name : String::from("Mouse"),
        price : 50.0
    };

    let product3 = Product {
        name : String::from("Keyboard"),
        price : 100.0
    };

    let mut cart = ShoppingCart::new();
    
    cart.add_product(product1);
    cart.add_product(product2);
    cart.add_product(product3);

    println!("Total: {}", cart.total());

    cart.remove_product("Mouse");

    println!("Total after removing mouse: {}", cart.total());
}