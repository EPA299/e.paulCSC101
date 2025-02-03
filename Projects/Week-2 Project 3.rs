fn main() {
    let amounts = [450_000.0, 1_500_000.0, 750_000.0, 2_850_000.0, 250_000.0];

    let sum = amounts[0] + amounts[1] + amounts[2] + amounts[3] + amounts[4]
    let count = 5; 
    let average = sum / count as f64;

    println!("Sum of sales amounts: {:.2}", sum);
    println!("Average of sales amounts: {:.2}", average);
}
