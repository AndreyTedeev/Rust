fn main() {
    let count = 5;
    for i in 0..count {
        println!("Hello, world!{0}", i);
    }
    println!("\nPress ENTER to exit");
    let mut s = String::new();
    match std::io::stdin().read_line(&mut s) {
        Ok(_) => println!("{}", s),
        Err(error) => println!("{}", error),
    };
}
