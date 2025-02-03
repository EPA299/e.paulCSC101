fn main() {

    let experienced = true;
    let age = 35;

    let incentive: f64;

    if experienced {
        if age >= 40 {
            incentive = 1_560_000.0;
        } else if age >= 30 {
            incentive = 1_480_000.0;
        } else if age < 28 {
            incentive = 1_300_000.0;
        } else {
            incentive = 1_480_000.0;
        }
    } else {
        incentive = 100_000.0;
    }
    
    println!("The annual incentive is: N{:.2}", incentive);
}
