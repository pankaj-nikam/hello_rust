fn main() {
    let mut pankaj = Person {
        first_name: String::from("Pankaj"),
        last_name: String::from("Nikam"),
        age: 32,
    };
    pankaj.first_name = "Amit".to_string();
    println!("{:#?}", pankaj);
}

#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
    age: u8,
}
