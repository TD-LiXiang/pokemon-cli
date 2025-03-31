use super::language::Language;
use clap::Parser;

/// Show Pokémon in the terminal
#[derive(Parser, Debug)]
#[command(name = "Pokémon CLI")]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Pokemon idx
    #[arg(short, long, value_parser = clap::value_parser!(u32).range(1..=905))]
    pub idx: Option<u32>,

    /// Show shiny version
    #[arg(short, long, default_value_t = false)]
    pub shiny: bool,

    /// Pokemon name language
    #[arg(short, long, value_enum, default_value_t = Language::Eng)]
    pub language: Language,
}
