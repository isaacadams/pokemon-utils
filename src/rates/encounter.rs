use crate::pokedex;
use rand::Rng;

const MYTHICAL: &'static [u16] = &[
    pokedex::MEW.index,
    pokedex::CELEBI.index,
    pokedex::JIRACHI.index,
    pokedex::DEOXYS.index,
];

const LEGENDARY: &'static [u16] = &[
    pokedex::MEWTWO.index,
    pokedex::LUGIA.index,
    pokedex::HOOH.index,
    pokedex::KYOGRE.index,
    pokedex::GROUDON.index,
    pokedex::RAYQUAZA.index,
];

const SUB_LEGENDARY: &'static [u16] = &[
    pokedex::ARTICUNO.index,
    pokedex::ZAPDOS.index,
    pokedex::MOLTRES.index,
    pokedex::RAIKOU.index,
    pokedex::ENTEI.index,
    pokedex::SUICUNE.index,
    pokedex::REGIROCK.index,
    pokedex::REGICE.index,
    pokedex::REGISTEEL.index,
    pokedex::LATIAS.index,
    pokedex::LATIOS.index,
];

// generates random number between 0 and 100
// lower numbers are more rare
// number determines which bucket to draw from
// then randomly draws pokemon from bucket
pub fn encounter_random_pokemon() -> u16 {
    // larger numbers are more rare
    let gen = generate_random_encounter();
    let bucket = match gen {
        8.. => {
            return *pick_random_item_from(&generate_all_none_rare_pokemon());
        }
        5..=7 => SUB_LEGENDARY,
        2..=4 => LEGENDARY,
        0..=1 => MYTHICAL,
    };

    *pick_random_item_from(bucket)
}

pub fn is_rare(no: &u16) -> bool {
    SUB_LEGENDARY.contains(no) || LEGENDARY.contains(no) || MYTHICAL.contains(no)
}

fn pick_random_item_from<T>(items: &[T]) -> &T {
    let length = items.len();
    let random_index = rand::thread_rng().gen_range(0..length);
    &items[random_index]
}

/// this will return a range of numbers from 0 to 100
/// 0 is rare
/// 100 is common
fn generate_random_encounter() -> u8 {
    let mut rng = rand::thread_rng();
    // adjust this value to control the distribution shape
    // the lower values make 100 more common
    let exponent = 0.55;

    // Generate a random float between 0.0 and 1.0 and apply the power-law distribution
    let random_float = rng.gen::<f64>();
    let result = (random_float.powf(exponent) * 100.0) as u8;

    result
}

fn generate_all_none_rare_pokemon() -> Vec<u16> {
    // include all up to last 3rd gen pokemon
    (pokedex::BULBASAUR.index..=pokedex::DEOXYS.index)
        // filter out rare pokemon
        .filter(|no| !is_rare(no))
        .collect()
}

#[cfg(test)]
mod test {

    use super::*;
    use std::collections::HashMap;

    #[test]
    fn none_rare_pokemon_generator_excludes_rares() {
        let common = generate_all_none_rare_pokemon();

        let mut does_not_contain_rare = true;

        MYTHICAL.iter().for_each(|no| {
            does_not_contain_rare &= !common.contains(no);
        });

        LEGENDARY.iter().for_each(|no| {
            does_not_contain_rare &= !common.contains(no);
        });

        SUB_LEGENDARY.iter().for_each(|no| {
            does_not_contain_rare &= !common.contains(no);
        });

        assert!(does_not_contain_rare);
    }

    #[test]
    fn simulate_pokemon_encounters() {
        let mut encounters = HashMap::new();
        for _ in 0..1_000 {
            let no = encounter_random_pokemon();
            encounters
                .entry(no)
                .and_modify(|v| *v += 1)
                .or_insert(1_usize);
        }

        for (key, value) in encounters {
            println!(
                "{} times encountered no #{}, is rare: {}",
                value,
                key,
                is_rare(&key)
            );
        }
    }

    #[test]
    fn random_encounter_distribution() {
        println!("{}", pick_random_item_from(MYTHICAL));

        let mut integers = Vec::new();
        let mut floats = Vec::new();

        for _ in 0..1_000 {
            let random_number = generate_random_encounter();
            integers.push(random_number);
            floats.push(random_number as f32);
        }

        plot(&floats).unwrap();

        println!("{} 8..", integers.iter().filter(|n| **n >= 8).count());
        println!(
            "{} 5..=7",
            integers.iter().filter(|n| **n >= 5 && **n <= 7).count()
        );
        println!(
            "{} 2..=4",
            integers.iter().filter(|n| **n >= 2 && **n <= 4).count()
        );
        println!("{} 0..=1", integers.iter().filter(|n| **n <= 1).count());
    }

    fn plot(nums: &[f32]) -> Result<(), Box<dyn std::error::Error>> {
        use plotters::coord::types::RangedCoordf32;
        use plotters::prelude::*;
        let root = BitMapBackend::new("plot.png", (640, 480)).into_drawing_area();

        root.fill(&RGBColor(240, 200, 200))?;

        let root = root.apply_coord_spec(Cartesian2d::<RangedCoordf32, RangedCoordf32>::new(
            0f32..100f32,
            0f32..1f32,
            (0..640, 0..480),
        ));

        let dot_and_label = |x: f32, y: f32| {
            return EmptyElement::at((x, y))
                + Circle::new((0, 0), 3, ShapeStyle::from(&BLACK).filled())
                /* + Text::new(
                    format!("({:.2},{:.2})", x, y),
                    (10, 0),
                    ("sans-serif", 15.0).into_font(),
                ) */;
        };

        nums.iter().for_each(|n| {
            root.draw(&dot_and_label(*n, 0.5)).unwrap();
        });

        root.present()?;
        Ok(())
    }
}
