mod assets;
mod cli;
mod pokemon;

use clap::Parser;
use crate::cli::args::Args;
use crate::pokemon::model::Pokemon;

fn main() {
    let Args {
        idx,
        shiny: is_force_shiny,
        language,
    } = Args::parse();
    Pokemon::new(idx, is_force_shiny).print(language);
}
