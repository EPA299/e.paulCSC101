use std::io;

fn main() {
    loop {
        println!("Select a geometric calculation:");
        println!("1. Area of Trapezium");
        println!("2. Area of Rhombus");
        println!("3. Area of Parallelogram");
        println!("4. Area of Cube");
        println!("5. Volume of Cylinder");
        println!("6. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: u32 = choice.trim().parse().expect("Please enter a valid number");

        if choice == 6 {
            break;
        }

        match choice {
            1 => calculate_area_of_trapezium(),
            2 => calculate_area_of_rhombus(),
            3 => calculate_area_of_parallelogram(),
            4 => calculate_area_of_cube(),
            5 => calculate_volume_of_cylinder(),
            _ => println!("Invalid choice, please try again."),
        }
    }
}

fn calculate_area_of_trapezium() {
    let inputs = input_values(3);
    let height = inputs[0];
    let base1 = inputs[1];
    let base2 = inputs[2];
    let area = (height / 2.0) * (base1 + base2);
    println!("The area of the trapezium is: {:.2}", area);
}

fn calculate_area_of_rhombus() {
    let inputs = input_values(2);
    let diagonal1 = inputs[0];
    let diagonal2 = inputs[1];
    let area = (diagonal1 * diagonal2) / 2.0;
    println!("The area of the rhombus is: {:.2}", area);
}

fn calculate_area_of_parallelogram() {
    let inputs = input_values(2);
    let base = inputs[0];
    let height = inputs[1];
    let area = base * height;
    println!("The area of the parallelogram is: {:.2}", area);
}

fn calculate_area_of_cube() {
    let inputs = input_values(1);
    let side = inputs[0];
    let area = 6.0 * (side * side);
    println!("The area of the cube is: {:.2}", area);
}

fn calculate_volume_of_cylinder() {
    let inputs = input_values(2);
    let radius = inputs[0];
    let height = inputs[1];
    let volume = std::f64::consts::PI * (radius * radius) * height;
    println!("The volume of the cylinder is: {:.2}", volume);
}

fn input_values(num: i32) -> Vec<f64> {
    let mut inputs = vec![0.0; num as usize];
    for i in 0..num {
        println!("Enter value {}: ", i + 1);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        inputs[i as usize] = input.trim().parse().expect("Please enter a valid number");
    }
    inputs
}
