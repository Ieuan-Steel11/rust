fn main() {
    println!("Hello, World!");
    // '!' means println is a macro not a function
    // main function like most c like languages
    // ';' to end expressions
}

// cargo is the pckage system for rust it is used instead of rustc
// cargo has a number of appealing features such as setting up projets with cargo new
// Important commands: 
//      cargo check - checks that code can actually compile
//      cargo build - compiles code into a binary that can be ran with ./[path to binary]
//      cargo run - compiles code into a binary then autonmatically runs it
//      ./target/debug/[project_name] - runs the code without rebuilding the binary has to be built to run
    