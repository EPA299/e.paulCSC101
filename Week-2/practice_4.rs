fn main() {
    let p: f64 = 1000.0;  // Principal amount
    let r: f64 = 1.0;     // Rate of interest
    let t: f64 = 2.0;     // Time in years

    // Simple Interest Calculation
    let a = p * (1.0 + (r / 100.0)) * t; // Final amount after interest
    println!("Amount is {}", a);

    let si = a - p; // Simple Interest
    println!("Simple Interest is {}", si);
}