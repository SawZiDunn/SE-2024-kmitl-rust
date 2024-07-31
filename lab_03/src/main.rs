use std::io;

fn main() {
    let mut warehouse: Vec<(u32, String, u32)> = Vec::new();

    loop {
        println!("WarehouseInventory Management:");
        println!("1. Add a new product (ensure unique ID)");
        println!("2. Update the stock quantity of an existing product.");
        println!("3. Remove a product by its ID.");
        println!("4. List all products in the inventory");
        println!("5. Exist program");

        let mut choice = String::new();
        println!("Please enter a choice");
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid Input. Please enter a number!");
                0
            }
        };

        match choice {
            1 => {
                let mut input = String::new();
                println!("Id:");
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read input");
                let id: u32 = input.trim().parse().expect("Falied to read input");
                input.clear();

                let mut id_exists = false;
                for (existing_id, _, _) in &warehouse {
                    if *existing_id == id {
                        id_exists = true;
                        break;
                    }
                }

                if id_exists {
                    println!("Product ID already exists.");
                    continue;
                }

                println!("Product Name:");
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read input");
                let name = input.trim().to_string();
                input.clear();

                println!("Quantity:");
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read input");
                let quantity: u32 = match input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Failed to read input!");
                        continue;
                    }
                };
                input.clear();

                warehouse.push((id, name, quantity));
                println!("Product added successfully");
            }
            2 => {
                // Update
                let mut input = String::new();
                println!("ID to update: ");
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read input");
                let id: u32 = input.trim().parse().expect("Falied to read input");
                input.clear();

                println!("Updated quantity: ");
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read input");
                let quantity: u32 = input.trim().parse().expect("Falied to read input");
                input.clear();

                let mut i = 0;
                while i < warehouse.len() {
                    if warehouse[i].0 == id {
                        warehouse[i].2 = quantity;
                        println!("Quantity updated successfully!");
                        break;
                    }
                    i += 1;
                }

                if i == warehouse.len() {
                    println!("Product not found!");
                }
            }
            3 => {
                // delete
                let mut input = String::new();
                println!("ID to delete: ");
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read input");
                let id: u32 = input.trim().parse().expect("Falied to read input");
                input.clear();

                let mut i = 0;
                let mut is_removed = false;
                while i < warehouse.len() {
                    if warehouse[i].0 == id {
                        warehouse.remove(i);
                        println!("Product removed successfully.");
                        is_removed = true;
                        break;
                    }
                    i += 1;
                }

                if !is_removed {
                    println!("Product not found!");
                }
            }
            4 => {
                // List All Products
                if warehouse.is_empty() {
                    println!("Warehouse is empty.");
                } else {
                    println!("Products in warehouse:");
                    for (id, name, quantity) in &warehouse {
                        println!("ID: {}, Name: {}, Stock: {}", id, name, quantity);
                    }
                }
                println!();
            }
            5 => {
                break;
            }
            _ => {
                println!("Wrong Choice!");
                println!();
                continue;
            }
        }
    }
}
