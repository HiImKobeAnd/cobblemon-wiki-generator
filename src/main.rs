mod pokemon;
mod pokemon_routes;

use std::string;

use askama::Template;
use axum::{extract::Path, response::Html, routing::get, Router};
use pokemon::Pokemon;
use pokemon_routes::PokemonRoutes;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/pokemon/:generation/:pokemon_name", get(get_pokemon));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn index() -> Html<String> {
    let mut pokemon_routes: PokemonRoutes = PokemonRoutes {
        name: vec![],
        generation: vec![],
    };

    if let Ok(mut current_generation_dir) = std::env::current_dir() {
        current_generation_dir.push("pokemon-jsons/generation1");

        if let Ok(pokemon_entries) = std::fs::read_dir(&current_generation_dir) {
            for pokemon_entry in pokemon_entries {
                if let Ok(pokemon_entry) = pokemon_entry {
                    let generation = String::from("generation1");
                    let pokemon_name = pokemon_entry
                        .file_name()
                        .into_string()
                        .expect("Cant convert to String");

                    pokemon_routes
                        .name
                        .push(pokemon_name.trim_end_matches(".json").to_string());
                    pokemon_routes.generation.push(generation);
                }
            }
        } else {
            println!("Error reading directory!");
        }
    } else {
        println!("Error getting current directory!");
    }

    Html(pokemon_routes.render().unwrap())
}

async fn get_pokemon(Path((generation, pokemon_name)): Path<(String, String)>) -> Html<String> {
    let abra_string =
        std::fs::read_to_string(format!("pokemon-jsons/{generation}/{pokemon_name}.json"))
            .expect("Unable to read file");
    let abra = serde_json::from_str::<Pokemon>(&abra_string).expect("msg");

    Html(abra.render().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pokemon_data_structure() {
        if let Ok(mut current_dir) = std::env::current_dir() {
            current_dir.push("pokemon-jsons/generation1");

            if let Ok(entries) = std::fs::read_dir(&current_dir) {
                for entry in entries {
                    if let Ok(entry) = entry {
                        let file_name = entry.file_name();
                        let pokemon_string =
                            std::fs::read_to_string(entry.path()).expect("Unable to read file");

                        println!("{:#?}", file_name);
                        match serde_json::from_str::<Pokemon>(&pokemon_string) {
                            Ok(parsed_pokemon) => {
                                // Successfully parsed JSON into Pokemon struct
                                // You can perform further checks or actions here if needed
                                println!("Succelly parsed: {:?}", parsed_pokemon);
                            }
                            Err(err) => {
                                // Print more detailed error information
                                println!("Error parsing {}: {}", file_name.to_string_lossy(), err);
                            }
                        }
                    }
                }
            } else {
                println!("Error reading directory!");
            }
        } else {
            println!("Error getting current directory!");
        }
    }
}
