use askama::Template;

#[derive(Debug, Template)]
#[template(path = "index.html")]

pub struct Generations {
    pub generation: Vec<PokemonRoutes>,
}

#[derive(Debug)]
pub struct PokemonRoutes {
    pub pokemon_name: Vec<String>,
    pub pokemon_generation: Vec<String>,
}
