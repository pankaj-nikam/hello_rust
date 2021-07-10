fn main() {
    println!("The addition of {} and {} is {}.", 22, 343, 22 + 343);
    /* Steps performed by rust for the operation below:
    First as usual, parenthesis are processed.
    Then we get 17 % 5 + 20 * 30 /7
    Then we multiply and divide. Since they are present in this expression, it is evaluated left to right.
    We get 17 % 5 + 600 / 7, which is 17 % 5 + 85, which in turn means 2 + 85 which makes it 87.*/
    println!("{}", (23 - 6) % 5 + 20 * 30 / (3 + 4));
    //Let us check floating variables:
    println!("The sum of {} and {} is {}", 90.5, 31.9, 90.5 + 31.9);
    //This does not round
    println!("The sum of {} and {} is {}", 80.3, 34.9, 80.3 + 34.9);
}