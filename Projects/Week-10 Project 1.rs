struct Laptop {
    brand: String,
    price_per_unit: f64,
    quantity: u32,
}

impl Laptop {
    fn total_cost(&self, purchase_quantity: u32) -> f64 {
        self.price_per_unit * purchase_quantity as f64
    }
}

fn main() {
    let hp_laptop = Laptop {
        brand: String::from("HP"),
        price_per_unit: 650_000.0,
        quantity: 10,
    };
    let ibm_laptop = Laptop {
        brand: String::from("IBM"),
        price_per_unit: 755_000.0,
        quantity: 6,
    };
    let toshiba_laptop = Laptop {
        brand: String::from("Toshiba"),
        price_per_unit: 550_000.0,
        quantity: 10,
    };
    let dell_laptop = Laptop {
        brand: String::from("Dell"),
        price_per_unit: 850_000.0,
        quantity: 4,
    };

    let purchase_quantity = 3;

    let total_cost = hp_laptop.total_cost(purchase_quantity)
        + ibm_laptop.total_cost(purchase_quantity)
        + toshiba_laptop.total_cost(purchase_quantity)
        + dell_laptop.total_cost(purchase_quantity);

    println!("Total cost for purchasing 3 laptops from each brand: N{:.2}", total_cost);
}
