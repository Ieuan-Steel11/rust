fn main() {
    println!("Hello, world!");
    // rust is a statically typed language that means the types of all variables must be known at compile time

    let g: u32 = "42".parse().expect("not a number");
    // .parse() converts strings to integers
    // .expect() is a testing function that prints out an error message if the wrong type is given for g
    // most of the time types can be inferred however when converting between types and it isn't clear a type must be a part of the declaration

    println!("the value of g is {g}");

    // a scalar type represents a single value and Rust has four primary scalar types: int, bool, float, char

    // integers can be signed or unsigned (doesn't need a sign because iit is obly ever positive) which decides whetehr it can be negative or not
    // 'i' prefix for signed 'u' for unsigned 
    //
    // i8     u8     
    // i16    u16
    // i32    u32
    // i64    u64
    // i128   u128
    // isize  usize

    // integer overflow happens when try to asign an integer a value outside of its range i.e let in: i8 = 258;
    // would cause the compiler to panic, you can change this setting with wrappers which will subtract the maximum value of the type from the assigned value

    // floating points are variables capable of holding decimal numbers
    // there are two flaot types f64 (default) and f32 both are about the same speedwise on modern cpu's while f64 is more precise being 64 bits instead of 32

    let dec = 2.01;
    println!("the value fo dec is {dec}");


    // rust supports the basic mathematical operations: addition, subtraction, multiplication, division, and remainder
    // integer division rounds to the nearest integer

    let sum1 = 23 + 3;
    let sum2 = 23 - 3;
    let sum3 = 23 * 3;
    let sum4 = 23 / 3; // outputs 7 because int division rounds to nearest int
    let sum5 = 23 % 3;

    println!("The value of the sums are: {sum1}, {sum2}, {sum3}, {sum4}, {sum5}");

    // rust supports the bool type

    let t = true;
    let tt: bool = false;

    println!("the value of t is {t}, the value of tt is {tt}");

    // rust supports the char type it's the most primitive alphabetical type 
    // the char type is 4 bytes and represents unicode scalar value and can be used for more than just ordinary letters

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    println!("Chars: {c}, {z}, {heart_eyed_cat}")

    // compound types are can group multiple values in one type like an array 
    // rust has two primitve compuind types: tuples and, arrays


}
