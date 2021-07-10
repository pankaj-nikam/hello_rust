fn main() {
    println!("The addition of {} and {} is {}.", 22, 343, 22 + 343);
    /* Steps performed by rust for the operation below:
    First as usual, parenthesis are processed.
    Then we get 17 % 5 + 20 * 30 /7
    Then we multiply and divide. Since they are present in this expression, it is evaluated left to right.
    We get 17 % 5 + 600 / 7, which is 17 % 5 + 85, which in turn means 2 + 85 which makes it 87.*/
    println!("{}", (23 - 6) % 5 + 20 * 30 / (3 + 4));
}