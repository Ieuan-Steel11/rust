fn main() {
    println!("Hello, world!");

    let x = 21;

    print_num(x);
    // call the fucntion called print_num and pass arguement x

    let y = six();
    // assign the returned value of the function six() the function to variable y

    println!("the value returned to by the function six() is {y}")
}

fn print_num(num: i32) {
    println!("the value of the arguement passed to the function is {num}");
    // you have to assign a type to any parametres but otherwise similar to python
    // statements are instructions that DO NOT return values and perform actions
    // expressions evaluate to a resulting value

    let _var = 1;
    // let is also a statement keyword therfore you can't stack let keyowrds because let doesn't return a value

    let _y = {
        let x = 1;
        x + 1
    };
    // example of an expression is creating a new scope block, calling a macro, calling a function
}

fn six() -> i32 {
    6
    // most functions return last expression implicity
    // the type of the value of the returned variable must be explicit and is shown with '->' and then the type
}