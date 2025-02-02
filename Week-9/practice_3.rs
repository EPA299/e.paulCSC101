use std::fs;

fn main() {
    // Attempt to remove the file named "data.txt"
    fs::remove_file("data.txt").expect("could not remove file");

    // Print a message if the file is successfully removed
    println!("File is removed");
}
