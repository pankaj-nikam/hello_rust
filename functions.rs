fn line() {
    println!("Line");
}

fn main() {
    fn something_else() {
        println!("Something else");
    }
    line();
    line();
    line();
    something_else();
}
