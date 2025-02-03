use std::fs::File;
use std::io::{self, Write};

fn main() {
    let categories = vec![
        ("Lager", vec!["33 Export", "Desperados", "Goldberg", "Gulder", "Heineken", "Star"]),
        ("Stout", vec!["Legend", "Turbo King", "Williams"]),
        ("Non-Alcoholic", vec!["Maltina", "Amstel Malta", "Malta Gold", "Fayrouz"]),
    ];

    display_categories(&categories);

    save_categories_to_file(&categories).expect("Failed to save categories to file");
}

fn display_categories(categories: &Vec<(&str, Vec<&str>)>) {
    println!("Nigerian Breweries Plc - Product Portfolio");
    for (category, brands) in categories {
        println!("\n{}:", category);
        for brand in brands {
            println!("  - {}", brand);
        }
    }
}

fn save_categories_to_file(categories: &Vec<(&str, Vec<&str>)>) -> io::Result<()> {
    let mut file = File::create("categories.txt")?;
    writeln!(file, "Nigerian Breweries Plc - Product Portfolio")?;
    for (category, brands) in categories {
        writeln!(file, "\n{}:", category)?;
        for brand in brands {
            writeln!(file, "  - {}", brand)?;
        }
    }
    Ok(())
}
