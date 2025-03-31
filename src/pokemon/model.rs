use super::{
    data::PokemonData,
    printer::PokemonPrinter,
    random::{check_shiny_probability, get_random_pokemon_idx},
};
use crate::{assets::get_pokemon_json_string, cli::language::Language};
use colored::Colorize;
use std::collections::HashMap;

pub struct Pokemon {
    pub idx: String,
    pub data: PokemonData,
    pub is_shiny: bool,
    pub image_path: String,
}

impl Pokemon {
    pub fn new(idx: Option<u32>, is_force_shiny: bool) -> Self {
        let idx_value = idx.unwrap_or_else(|| get_random_pokemon_idx(905));
        let idx_key = format!("{:03}", idx_value);

        let data = get_pokemon_json_string();
        let pokemon_map: HashMap<String, PokemonData> =
            serde_json::from_str(data.as_str()).expect("Reading Pokemon Data failed.");
        let pokemon_data = pokemon_map
            .get(idx_key.as_str())
            .expect(format!("Pokemon #{} not found.", idx_value).as_str());

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

    pub fn print(&self, language: Language) {
        PokemonPrinter::new(self.image_path.clone()).print();
        let pokemon_name = self.get_name(language);
        let name = if self.is_shiny {
            format!("{} {}", pokemon_name, "🌟")
        } else {
            pokemon_name
        };

        println!("#{} - {}", self.idx, name.magenta());
    }

    fn get_name(&self, language: Language) -> String {
        match language {
            Language::JpnRo => self.data.name.jpn_ro.clone(),
            Language::Jpn => self.data.name.jpn.clone(),
            Language::Chs => self.data.name.chs.clone(),
            _ => self.data.name.eng.clone(),
        }
    }
}
