fn main() {
    println!("Hello, world!");
    // rust is a statically typed language that means the types of all variables must be known at compile time

    let g: u32 = "42".parse().expect("not a number");
    // .parse() converts strings to integers
    // .expect() is a testing function that prints out an error message if the wrong type is given for g
    // most of the time types can be inferred however when converting between types and it isn't clear a type must be a part of the declaration

    println!("the value of g is {g}");

}
