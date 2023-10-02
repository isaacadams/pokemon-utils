use pokemon_utils::{self, is_rare, PokedexEntry};

fn main() {
    let pokemon = pokemon_utils::encounter_random_pokemon();
    // one way to convert pokemon index into an entry
    let _: &PokedexEntry = PokedexEntry::get_by_id(pokemon as usize).unwrap();
    // another way to do the conversion
    let entry: &PokedexEntry = pokemon.try_into().unwrap();
    println!("found a wild {}!", entry.name);
    println!("is rare? {}", is_rare(&entry.index));
}
