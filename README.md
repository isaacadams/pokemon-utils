# Pokemon Utils

a collection of pokemon related utilities

```rust
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
```

## Resources

- https://msikma.github.io/pokesprite/index.html
  - water type icon example: https://raw.githubusercontent.com/msikma/pokesprite/master/misc/types/masters/water.png
- pokeapi
  - explorer app: https://unpkg.com/css-chain-test@1.1.9/src/PokeApi-Explorer.html
  - github: https://github.com/PokeAPI/sprites
  - pokemon api example: https://pokeapi.co/api/v2/pokemon/1
  - pikachu sprite examples
    - https://raw.githubusercontent.com/PokeAPI/sprites/master/sprites/pokemon/25.png
    - https://raw.githubusercontent.com/PokeAPI/sprites/master/sprites/pokemon/versions/generation-ii/gold/transparent/25.png
    - https://raw.githubusercontent.com/PokeAPI/sprites/master/sprites/pokemon/versions/generation-iii/emerald/25.png
