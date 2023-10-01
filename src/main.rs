#[allow(dead_code)]
mod pokedex;

fn main() {
    println!("Hello, world!");
    assert_eq!(pokedex::BULBASAUR.index, 1);
    assert_eq!(pokedex::MEWTWO.index, 150);
}
