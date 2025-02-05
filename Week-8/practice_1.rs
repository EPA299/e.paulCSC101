fn main() {
    // Using Vec::new()
    let v: Vec<i64> = Vec::new();

    // Printing the size of the vector
    println!("\nThe length of Vec::new is: {}", v.len());

    // Using macro
    let v = vec!["Grace", "Effiong", "Basil", "Kareem", "Susan"];

    // Printing the size of the vector
    println!("\nThe length of vec macro is: {}", v.len());
}
