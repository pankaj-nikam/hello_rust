fn main() {
    let mut pankaj = Person {
        first_name: "Pankaj".to_string(),
        last_name: "Nikam".to_string(),
        age: 32,
    };
    pankaj.first_name = "Amit".to_string();
    println!("{:#?}", pankaj.first_name);
}

struct Person {
    first_name: String,
    last_name: String,
    age: u8,
}
