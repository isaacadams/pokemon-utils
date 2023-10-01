#[allow(dead_code)]
mod pokedex;
mod rates;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn pokedex_is_valid() {
        assert_eq!(pokedex::BULBASAUR.index, 1);
        assert_eq!(pokedex::MEWTWO.index, 150);
    }
}
