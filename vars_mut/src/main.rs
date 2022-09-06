fn main() {
    let x = 5;
    // use let to declare a variable

    println!("the value of x is {x}");
    // {} to insert a variable into the println! macro
    // x = 6; won't work because the variable is immutable
    // if you don't use a varibale it will produce a warning about an unused variable

    let mut y = 10;
    println!("the value fo y is {y}");
    // declaring a mutable variable y

    y = 9;
    println!("the value of y is {y}");
    // this works however, because the mut keyword allows a variable to be mutable 
    // i.e having a value that can change
    // you can't mutate a variable's type:
    //      i.e let mut z = 4;
    //          let z = 'a';

    let a = 1;
    println!("the value of a is {a}");
    // decalring new variable 'a'

    let a = 2;
    println!("the value of a is {a}");
    // this called shadowing where you 
    // create a new variable of the same name that replaces the old one

    {
        let a = a * 5;
        println!("the value of a is {a}");
        // this is inner shadowing it allows you to use the old variable in the decalration of the new variable like above
        // the value only changes within the scope so it will return 2 outside the scope
    }

    println!("the value of a is {a}");
    // the value returns to 2 outside the innerscope

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("the number of seconds in 3 hours is {THREE_HOURS_IN_SECONDS}");
    // constants are immutable by default no mut keywoard 
    // uses const instead of let
    // constants are different from immutable variables
    // constants are immutable by default and have to have data type assigned at decalration
    // constants can be decalred at any scope
    // constantscan only be set to a constant value not result of a value computed at runtime

}
