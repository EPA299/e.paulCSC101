fn main() {
    let v = vec![20, 40, 60, 80]; // vector v owns the object in heap

    let v2 = v; // ownership transferred to v2
    let v2_return = display(v2); // ownership transferred to display function and back to v2_return

    println!("In main {:?}", v2_return); // print the returned vector
}

fn display(v: Vec<i32>) -> Vec<i32> {
    println!("inside display {:?}", v); // print the vector inside display function
    return v; // return the vector
}
