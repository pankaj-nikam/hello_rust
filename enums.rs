fn main() {
    let contin = Continent::Oceania;

    match contin {
        Continent::Europe => println!("E"),
        Continent::Asia => println!("As"),
        Continent::Africa => println!("Af"),
        Continent::America => println!("Am"),
        Continent::Oceania => println!("O"),
    }

    let cardinal = CardinalPoint::North;
    match cardinal {
        CardinalPoint::East => println!("Sun rises in east."),
        CardinalPoint::West => println!("Sun sets in west"),
        _ => println!("I dont know where to go!"),
    }

    //Let us see some guards in enums:
    for n in -5..5 {
        println!(
            "{} is {}",
            n,
            match n {
                0 => "Zero",
                1 => "One",
                _ if n < 0 => "Negative",
                _ => "Plural",
            }
        )
    }
}

enum CardinalPoint {
    North,
    South,
    East,
    West,
}

enum Continent {
    Europe,
    Asia,
    Africa,
    America,
    Oceania,
}
