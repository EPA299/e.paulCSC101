fn main() {
    let menu = [
        ("Poundo Yam/Edinkaiko Soup", 3200.0),
        ("Fried Rice & Chicken", 3000.0),
        ("Amala & Ewedu Soup", 2500.0),
        ("Eba & Egusi Soup", 2000.0),
        ("White Rice & Stew", 2500.0),
    ];

    let mut total = 0.0;

    println!("Menu:");
    println!("P = Poundo Yam/Edinkaiko Soup - N3,200");
    println!("F = Fried Rice & Chicken - N3,000");
    println!("A = Amala & Ewedu Soup - N2,500");
    println!("E = Eba & Egusi Soup - N2,000");
    println!("W = White Rice & Stew - N2,500");

    loop {
        let mut food_type = String::new();
        println!("Enter the food type (P, F, A, E, W) or Q to quit: ");
        std::io::stdin().read_line(&mut food_type).expect("Failed to read line");

        let food_type = food_type.trim().to_uppercase();
        if food_type == "Q" {
            break;
        }

        let mut quantity = String::new();
        println!("Enter the quantity: ");
        std::io::stdin().read_line(&mut quantity).expect("Failed to read line");

        let quantity: i32 = match quantity.trim().parse() {
            Ok(q) => q,
            Err(_) => {
                println!("Invalid quantity. Please enter a valid number.");
                continue;
            }
        };

        let price = match food_type.as_str() {
            "P" => 3200.0,
            "F" => 3000.0,
            "A" => 2500.0,
            "E" => 2000.0,
            "W" => 2500.0,
            _ => {
                println!("Invalid food type. Please try again.");
                continue;
            }
        };

        total += price * quantity as f64;
    }

    if total > 10_000.0 {
        total *= 0.95;  
    }

    println!("Total charges: N{:.2}", total);
}
