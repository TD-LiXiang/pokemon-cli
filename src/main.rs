mod app;
mod assets;
mod pokemon;
mod pokemon_data;
mod pokemon_printer;
mod random;

use app::App;
use pokemon::Pokemon;

fn main() {
    let app = App::new();
    let idx = app.idx.as_str();
    let is_force_shiny = app.is_force_shiny;
    let language = app.language;

    Pokemon::new(idx, is_force_shiny).print(language);
}
