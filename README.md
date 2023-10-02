# Pokemon Utils

a collection of pokemon related utilities

```rust
use pokemon_utils::{self, is_rare, PokedexEntry};

fn main() {
    let pokemon = pokemon_utils::encounter_random_pokemon();
    let pokemon = PokedexEntry::get_by_id(pokemon as usize).unwrap();
    println!("found a wild {}!", pokemon.name);
    println!("is rare? {}", is_rare(&pokemon.index));
}
```
