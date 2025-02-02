fn main() {
    // Initialize an array with city names
    let city_arr: [&str; 5] = ["Abuja", "Portharcourt", "Maiduguri", "Kano", "Lagos"];
    
    // Print the array and its size
    println!("array is {:?}", city_arr);
    println!("array size is: {}", city_arr.len());
    
    // Iterate over the array indices and print each city name with its index
    for index in 0..5 {
        println!("City index {} is located in: {}", index, city_arr[index]);
    }
}
