use std::vec;

fn main() {
    //     println!("Hello, world!");

    //     let t: i32 = 5;
    //     let t: i32 = t + 1;
    //     println!("t: {t}");

    // STRINGS
    let mut name = String::from("Gbenga philip");
    name.push_str("philip");
    println!("My name is : {name}");

    let men = "phil me";
    // name.push_str("gbenga");
    print!("name: {men}");
    // name = "gbenga";

    // Arrays
    let arr = [1, 2, 3, 4, 5];
    let arr2 = [arr[0]];
    let one = arr[0];
    println!("First element of the array: {}", one);
    println!("Array: {:?}", arr);
    println!("Array2: {:?}", arr2);

    // Vectors
    let vec = vec![1, 2, 3, 4, 5];
    let first_element = vec[0];
    let add = vec.clone();
    print!("Added {:?}", add);
    println!("First element of the vector: {:?}", first_element);
    print!("Vector: {:?}", vec);

    // Tuples
    let tuple = ("Trial", 2, 3);
    let types = tuple.0;
    println!("Tuple first element: {}", types);
    let (name, age, height) = tuple;
    println!("Tuple: {:?}", tuple);
    println!("Name: {}, Age: {}, Height: {}", name, age, height);
}
