use std::io;

fn main() {
    let mut input = String::new();
    
    println!("Enter the value of a: ");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let a: f64 = input.trim().parse().expect("Please enter a valid number");
    input.clear();
    
    println!("Enter the value of b: ");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let b: f64 = input.trim().parse().expect("Please enter a valid number");
    input.clear();
    
    println!("Enter the value of c: ");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let c: f64 = input.trim().parse().expect("Please enter a valid number");
    
    let discriminant = b * b - 4.0 * a * c;
    
    if discriminant > 0.0 {
        
        let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let root2 = (-b - discriminant.sqrt()) / (2.0 * a);
        println!("The roots are real and distinct: x1 = {:.2}, x2 = {:.2}", root1, root2);
    } else if discriminant == 0.0 {
        
        let root = -b / (2.0 * a);
        println!("The root is real and repeated: x = {:.2}", root);
    } else {

        println!("There are no real roots.");
    }
}
