use crate::random::get_random_pokemon_idx;
use clap::{App as ClapApp, Arg};

pub struct App {
    pub idx: String,
    pub is_force_shiny: bool,
    pub language: String,
}

impl App {
    const IDX: &'static str = "idx";
    const SHINY: &'static str = "shiny";
    const LANGUAGE: &'static str = "language";
    const DEFAULT_LANGUAGE: &'static str = "eng";
    const ALLOWED_LANGUAGES: &'static [&'static str] = &["eng", "chs", "jpn", "jpn_ro"];

    pub fn new() -> Self {
        let matches = ClapApp::new("Pokemon CLI")
            .version("1.0")
            .author("Xiang Li")
            .about("Does awesome things")
            .arg(
                Arg::with_name(App::IDX)
                    .short("i")
                    .long("idx")
                    .required(false)
                    .takes_value(true)
                    .help("Pokemon idx"),
            )
            .arg(
                Arg::with_name(App::SHINY)
                    .short("s")
                    .long("shiny")
                    .help("Show shiny version"),
            )
            .arg(
                Arg::with_name(App::LANGUAGE)
                    .short("l")
                    .long("language")
                    .takes_value(true)
                    .default_value(App::DEFAULT_LANGUAGE)
                    .possible_values(App::ALLOWED_LANGUAGES)
                    .help("Pokemon name language"),
            )
            .get_matches();

        let idx_value = matches.value_of(App::IDX);
        let is_force_shiny = matches.is_present(App::SHINY);
        let language = matches.value_of(App::LANGUAGE).unwrap().to_string();

        let idx = if idx_value.is_none() {
            let random_idx = get_random_pokemon_idx(905);
            format!("{:03}", random_idx)
        } else {
            format!("{:03}", idx_value.unwrap().parse::<u32>().unwrap())
        };

        App {
            idx,
            is_force_shiny,
            language,
        }
    }
}
