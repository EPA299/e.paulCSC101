fn main() {
    let amounts = [450_000.00, 1_500_000.00, 750_000.00, 2_850_000.00, 250_000.00];
    
    let sum: f64 = amounts.iter().sum();
    
    let average = sum / amounts.len() as f64;
    
    println!("Sum of sales amounts: {:.2}", sum);
    println!("Average of sales amounts: {:.2}", average);
}
