fn main() {
    let hexadecimal = 0x10;
    let decimal = 10;
    let octal = 0o10;
    let binary = 0b10;
    println!("{} {} {} {}", hexadecimal, decimal, octal, binary);
    let one_crore = 1_00_00_000;
    println!("One crore: {}", one_crore);
    let scientific_notation = 1e8;
    println!("Scientific notation: {}", scientific_notation);
    let thirteen_millionths = 13e-6;
    println!("Thirteen Millionths {}", thirteen_millionths);
}
