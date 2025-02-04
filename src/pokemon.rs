use crate::pokemon_data::PokemonData;
use crate::pokemon_printer::PokemonPrinter;
use crate::random::check_shiny_probability;
use colored::Colorize;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

pub struct Pokemon {
    pub idx: String,
    pub data: PokemonData,
    pub is_shiny: bool,
    pub image_path: String,
}

impl Pokemon {
    pub fn new(idx: &str, is_force_shiny: bool) -> Self {
        let file = File::open("assets/pokemon.json").expect("Pokemon Data not found.");
        let reader = BufReader::new(file);
        let pokemons: HashMap<String, PokemonData> =
            serde_json::from_reader(reader).expect("Reading Pokemon Data failed.");
        let pokemon_data = pokemons
            .get(idx)
            .expect(format!("Pokemon #{} not found.", idx).as_str());

        let is_shiny = if is_force_shiny {
            true
        } else {
            check_shiny_probability(0.001)
        };

        let image_path = format!(
            "assets/{}/{}.png",
            if is_shiny { "shiny" } else { "regular" },
            pokemon_data.slug.eng
        );

        Pokemon {
            idx: pokemon_data.idx.clone(),
            data: pokemon_data.clone(),
            is_shiny,
            image_path,
        }
    }

    pub fn print(&self, language: String) {
        PokemonPrinter::new(self.image_path.clone()).print();
        let pokemon_name = self.get_name(language.as_str());
        let name = if self.is_shiny {
            format!("{} {}", pokemon_name, "🌟")
        } else {
            pokemon_name
        };

        println!("#{} - {}", self.idx, name.magenta());
    }

    fn get_name(&self, language: &str) -> String {
        match language {
            "jpn_ro" => self.data.name.jpn_ro.clone(),
            "chs" => self.data.name.chs.clone(),
            "jpn" => self.data.name.jpn.clone(),
            _ => self.data.name.eng.clone(),
        }
    }
}
