fn main() {
    // Name vector
    let name = vec!["Mary", "Sam", "Sally", "Greg", "Ade", "Mark", "June", "Ife"];

    // Age vector
    let age = vec![16, 17, 19, 22, 20, 21, 18, 23];

    println!("\nAge allocation:");

    // Loop to iterate through the elements in the vector
    for i in 0..age.len() {
        // Iterating through the vector using index i
        println!("{} is {} years old", name[i], age[i]);
    }
}
