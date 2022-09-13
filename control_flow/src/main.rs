fn main() {
    if_expr(6);
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

}
