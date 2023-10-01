macro_rules! define_pokemon_types {
    ($($variant:ident),*) => {
        #[derive(Debug)]
        pub enum PokemonType {
            Unknown(String),
            $($variant),*
        }

        paste::paste! {
            impl PokemonType {
                pub fn icon(&self) -> &'static str {
                    match self {
                        PokemonType::Unknown(_) => "https://raw.githubusercontent.com/PokeAPI/sprites/master/sprites/pokemon/0.png",
                        $(PokemonType::$variant => concat!("https://raw.githubusercontent.com/msikma/pokesprite/master/misc/types/masters/", stringify!([<$variant:lower>]), ".png"),)*
                    }
                }
            }

            impl From<&str> for PokemonType {
                fn from(value: &str) -> Self {
                    match value.to_lowercase().as_str() {
                        $(stringify!([<$variant:lower>]) => PokemonType::$variant,)*
                        _ => {
                            log::error!("unknown pokemon type {}", value);
                            PokemonType::Unknown(value.to_string())
                        },
                    }
                }
            }
        }
    }
}

define_pokemon_types!(
    Dark, Electric, Fairy, Fighting, Ground, Ice, Normal, Poison, Psychic, Rock, Steel, Water,
    Fire, Dragon, Bug, Flying, Ghost, Grass
);
