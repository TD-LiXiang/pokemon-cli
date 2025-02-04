use crate::assets::get_pokemon_json_string;
use crate::pokemon_data::PokemonData;
use crate::pokemon_printer::PokemonPrinter;
use crate::random::check_shiny_probability;
use colored::Colorize;
use std::collections::HashMap;

pub struct Pokemon {
    pub idx: String,
    pub data: PokemonData,
    pub is_shiny: bool,
    pub image_path: String,
}

impl Pokemon {
    pub fn new(idx: &str, is_force_shiny: bool) -> Self {
        let data = get_pokemon_json_string();
        let pokemon_map: HashMap<String, PokemonData> =
            serde_json::from_str(data.as_str()).expect("Reading Pokemon Data failed.");
        let pokemon_data = pokemon_map
            .get(idx)
            .expect(format!("Pokemon #{} not found.", idx).as_str());

        let is_shiny = if is_force_shiny {
            true
        } else {
            check_shiny_probability(0.001)
        };

        let image_path = format!(
            "{}/{}.png",
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
