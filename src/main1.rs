fn main() {
    let mut name: &str = "Bastian";
    let language: &str = "Rust";

    println!("Hallo, {}! Du programmierst gerade in {}!", name, language);

    name = "Stan";
    println!("Hallo, {}! Du programmierst gerade in {}!", name, language);
}