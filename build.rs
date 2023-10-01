use std::fs;
use std::io::prelude::*;
use std::process::Command;

fn main() {
    create_pokedex();
    Command::new("cargo").arg("fmt").spawn().unwrap();
}

fn create_pokedex() {
    let file = fs::read_to_string("./pokemon.txt").unwrap();
    let mut all_pokemon = file.lines();

    let file = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("src/pokedex.rs")
        .unwrap();

    let mut stream = std::io::BufWriter::new(file);

    stream
        .write(
            "
#[derive(Debug,PartialEq)]
pub struct PokedexEntry {
    pub index: u16,
    pub name: &'static str,
}\n"
            .as_ref(),
        )
        .unwrap();

    let mut identifiers = Vec::new();

    let mut index: u16 = 1;
    loop {
        let pokemon = all_pokemon.next();
        if pokemon.is_none() {
            break;
        }

        let pokemon = pokemon.unwrap();
        let (entry, identifier) = create_pokedex_entry(index, pokemon);
        identifiers.push(identifier);
        stream.write(entry.as_ref()).unwrap();
        index += 1;
    }

    stream
        .write(format!("pub const MAX: usize = {};", identifiers.len()).as_ref())
        .unwrap();

    stream
        .write(
            format!(
                "pub const ENTRIES: &'static [&PokedexEntry; {}] = &[{}];",
                identifiers.len(),
                identifiers.join(",")
            )
            .as_ref(),
        )
        .unwrap();

    stream.flush().unwrap();
}

fn create_pokedex_entry(index: u16, name: &str) -> (String, String) {
    let identifier = name
        // Mr. Mime, Type: Null
        .replace(" ", "_")
        // [-]: Nidoran-M / Nidoran-F
        // [']: Farfetch'd
        // [:]: Type: Null
        // [.]: Mr. Mime
        .replace(&['-', '\'', ':', '.'], "")
        .to_uppercase();
    (
        format!(
            "pub const {}: &'static PokedexEntry = &PokedexEntry {{ index: {}, name: \"{}\" }};\n",
            identifier, index, name
        ),
        identifier,
    )
}
