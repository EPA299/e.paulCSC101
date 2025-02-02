fn main() {
    // Vector v owns the object in heap memory
    let v = vec![101, 250, 330, 400];

    // Only a single variable can own the heap memory at any given time
    let v2 = v;

    // Trying to use v here would result in a compile-time error
    // as v's ownership has been transferred to v2
    println!("{:?}", v2);
}
