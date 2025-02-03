fn main() {
    let principal: f64 = 200_000.0;
    let rate: f64 = 12.0;
    let years: u32 = 5;

    let amount = principal * (1.0 + rate / 100.0).powi(years as i32);
    let compound_interest = amount - principal;

    println!("The compound interest for {} years is: ${:.2}", years, compound_interest);
}
