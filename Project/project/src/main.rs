use std::io::{self, Write};

#[derive(Debug)]
struct Product {
    name: String,
    description: String,
    price: f64,
    quantity: u32,
}


#[derive(Debug)]
enum InventoryError {
    NotFound,
    InvalidInput,
}

struct Inventory {
    products: Vec<Product>,
}

impl Inventory {
    fn add_product(&mut self, product: Product) {
        self.products.push(product);
    }

    fn edit_product(&mut self, name: &str, new_product: Product) -> Result<(), InventoryError> {
        for product in &mut self.products {
            if product.name == name {
                *product = new_product;
                return Ok(());
            }
        }
        Err(InventoryError::NotFound)
    }

    fn delete_product(&mut self, name: &str) -> Result<(), InventoryError> {
        if let Some(index) = self.products.iter().position(|p| p.name == name) {
            self.products.remove(index);
            Ok(())
        } else {
            Err(InventoryError::NotFound)
        }
    }

    fn generate_report(&self) -> String {
        let mut report = String::new();
        report.push_str("Inventory Report:\n");
        report.push_str("Name\tDescription\tPrice\tQuantity\n");
        for product in &self.products {
            report.push_str(&format!(
                "{}\t{}\t${:.2}\t{}\n",
                product.name, product.description, product.price, product.quantity
            ));
        }
        report
    }
}

fn authenticate(username: &str, password: &str) -> bool {
    let authorized_username = "admin";
    let authorized_password = "password123";

    if username == authorized_username && password == authorized_password {
        true
    } else {
        false
    }
}


fn main() {
    let mut inventory = Inventory {
        products: Vec::new(),
    };

    loop {
        println!("Welcome to the Inventory Management System");
        println!("1. Add Product");
        println!("2. Edit Product");
        println!("3. Delete Product");
        println!("4. Generate Report");
        println!("5. Exit");

        print!("Please enter your choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };

        match choice {
            1 => {
                println!("Adding Product:");
                print!("Enter product name: ");
                io::stdout().flush().unwrap();
                let mut name = String::new();
                io::stdin().read_line(&mut name).unwrap();

                print!("Enter product description: ");
                io::stdout().flush().unwrap();
                let mut description = String::new();
                io::stdin().read_line(&mut description).unwrap();

                print!("Enter product price: ");
                io::stdout().flush().unwrap();
                let mut price = String::new();
                io::stdin().read_line(&mut price).unwrap();
                let price: f64 = match price.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid input. Please enter a number.");
                        continue;
                    }
                };

                print!("Enter product quantity: ");
                io::stdout().flush().unwrap();
                let mut quantity = String::new();
                io::stdin().read_line(&mut quantity).unwrap();
                let quantity: u32 = match quantity.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid input. Please enter a number.");
                        continue;
                    }
                };

                let product = Product {
                    name: name.trim().to_string(),
                    description: description.trim().to_string(),
                    price,
                    quantity,
                };

                inventory.add_product(product);
                println!("Product added successfully!");
            }
            2 => {
                println!("Editing Product:");
                print!("Enter product name to edit: ");
                io::stdout().flush().unwrap();
                let mut name = String::new();
                io::stdin().read_line(&mut name).unwrap();

                if let Some(index) = inventory.products.iter().position(|p| p.name == name.trim()) {
                    print!("Enter new product name: ");
                    io::stdout().flush().unwrap();
                    let mut new_name = String::new();
                    io::stdin().read_line(&mut new_name).unwrap();

                    print!("Enter new product description: ");
                    io::stdout().flush().unwrap();
                    let mut new_description = String::new();
                    io::stdin().read_line(&mut new_description).unwrap();

                    print!("Enter new product price: ");
                    io::stdout().flush().unwrap();
                    let mut new_price = String::new();
                    io::stdin().read_line(&mut new_price).unwrap();
                    let new_price: f64 = match new_price.trim().parse() {
                        Ok(num) => num,
                        Err(_) => {
                            println!("Invalid input. Please enter a number.");
                            continue;
                        }
                    };

                    print!("Enter new product quantity: ");
                    io::stdout().flush().unwrap();
                    let mut new_quantity = String::new();
                    io::stdin().read_line(&mut new_quantity).unwrap();
                    let new_quantity: u32 = match new_quantity.trim().parse() {
                        Ok(num) => num,
                        Err(_) => {
                            println!("Invalid input. Please enter a number.");
                            continue;
                        }
                    };

                    let new_product = Product {
                        name: new_name.trim().to_string(),
                        description: new_description.trim().to_string(),
                        price: new_price,
                        quantity: new_quantity,
                    };

                    inventory.products[index] = new_product;
                    println!("Product edited successfully!");
                } else {
                    println!("Product not found.");
                }
            }
            3 => {
                println!("Deleting Product:");
                print!("Enter product name to delete: ");
                io::stdout().flush().unwrap();
                let mut name = String::new();
                io::stdin().read_line(&mut name).unwrap();

                match inventory.delete_product(&name.trim()) {
                    Ok(_) => println!("Product deleted successfully!"),
                    Err(InventoryError::NotFound) => println!("Product not found."),
                    Err(_) => println!("Error occurred while deleting product."),
                }
            }
            4 => {
                println!("Generating Report:");
                println!("{}", inventory.generate_report());
            }
            5 => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid choice. Please enter a number between 1 and 5."),
        }
    }
}
