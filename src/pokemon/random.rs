use rand::Rng;

pub fn get_random_pokemon_idx(max: u32) -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..=max)
}

pub fn check_shiny_probability(probability: f64) -> bool {
    let mut rng = rand::thread_rng();
    rng.gen_bool(probability)
}
