use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct PokemonName {
    pub eng: String,
    pub chs: String,
    pub jpn: String,
    pub jpn_ro: String,
}

#[derive(Deserialize, Clone)]
pub struct PokemonSlug {
    pub eng: String,
}

#[derive(Deserialize, Clone)]
pub struct PokemonData {
    pub idx: String,
    pub name: PokemonName,
    pub slug: PokemonSlug,
}
