fn line() {
    println!("Line");
}

fn add_two_numbers(num1: f64, num2: f64) {
    println!(
        "First number is {} and second number is {}, their sum is {}",
        num1,
        num2,
        num1 + num2
    );
}

fn print_double(mut num: f64) {
    num *= 2.0;
    println!("The double value is {}", num);
}

fn main() {
    fn something_else() {
        println!("Something else");
    }
    line();
    line();
    line();
    something_else();
    add_two_numbers(3.14, 2.22);
    let num = 10.0;
    print_double(num);
    println!("{}", num);
}
