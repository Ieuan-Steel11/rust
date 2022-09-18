fn main() {
    if_expr(6);
    for_loop()
    while_loop()
}

fn if_expr(num: i32) {
    if num < 5 {
        println!("condition true")
    } else {
        println!("condition false")
        // all other conditions
    }

    if num != 0 {
        println!("if num isn't 0")
        // if num won't work because rust won't try to convert to bool from non-bool
    }

    let condition = false;

    let x = if condition {5} else {6};
    // both resulting values from the conditio+n must be of the same type

    println!("the value of x is {x}")

}

fn for_loop() {

    let mut counter = 0;

     let result = loop {
        counter += 1;

        if counter == 25 {
            break 
            counter * 2
            // returns counter * 2 as a result as the block evaluates to the last condition i.e the one after the 'break'
        }
    };

    println!("The resulting value from the loop is {result}");

    let mut count = 0;

    'outer_loop: loop {

        println!("the value of count is {count}");

        let mut down_count = 10;

        loop {
            println!("remaining = {down_count}");

            if down_count == 7 {
                break;
                // if there four remaining iterqations of the inner loop
            }

            if count == 2 {
                break 'outer_loop;
                // if the inner loop has been run 5 times
            }
            down_count -= 1;
        }
        count += 1;
    };

    println!("End count = {count}")

}

fn while_loop() -> i32 {
    0
}
