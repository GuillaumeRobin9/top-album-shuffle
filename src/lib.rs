use wasm_bindgen::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;

// Expose this function to JavaScript
#[wasm_bindgen]
pub fn shuffle_album_from_csv(csv_content: &str) -> String {
    // Crée un vecteur pour stocker les albums
    let mut albums = Vec::new();

    // Divise le CSV en lignes
    for line in csv_content.lines().skip(1) { // Skip the header row
        let fields: Vec<&str> = line.split(',').collect();
        if fields.len() == 4 {
            let album = format!(
                "Album: {}, Artist: {}, Year: {}",
                fields[1], fields[2], fields[3]
            );
            albums.push(album);
        }
    }

    // Choisir un album aléatoire
    let mut rng = thread_rng();
    let chosen_album = albums.choose(&mut rng);

    // Retourner l'album choisi ou un message par défaut
    match chosen_album {
        Some(album) => album.to_string(),
        None => "No album found".to_string(),
    }
}
