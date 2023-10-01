use pokedex::PokedexEntry;

#[allow(dead_code)]
mod pokedex;
mod rates;
mod types;

impl PokedexEntry {
    pub fn get_by_id(no: usize) -> Option<&'static PokedexEntry> {
        if no > pokedex::MAX {
            return None;
        }

        Some(pokedex::ENTRIES[no - 1])
    }
}

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

    #[test]
    fn pokedex_entries_return_correct_pokemon() {
        assert_eq!(pokedex::BULBASAUR, PokedexEntry::get_by_id(1).unwrap());
        assert_eq!(pokedex::MEWTWO, PokedexEntry::get_by_id(150).unwrap());
    }

    #[test]
    fn pokedex_entries_returns_none_when_greater_than_max() {
        assert!(PokedexEntry::get_by_id(pokedex::MAX + 1).is_none());
    }
    #[test]
    fn pokedex_entries_returns_some_if_max() {
        assert!(PokedexEntry::get_by_id(pokedex::MAX).is_some());
    }
}
