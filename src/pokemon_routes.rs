use askama::Template;

#[derive(Debug, Template)]
#[template(path = "index.html")]
pub struct PokemonRoutes {
    pub name: Vec<String>,
    pub generation: Vec<String>,
}
