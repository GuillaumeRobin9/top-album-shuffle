use wasm_bindgen::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;

// Expose this function to js 
#[wasm_bindgen]
pub fn shuffle_album_from_csv(csv_content: &str) -> String {
    // store the album in a vector
    let mut albums = Vec::new();

    // get every line of the CSV
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

    // take a rand album
    let mut rng = thread_rng();
    let chosen_album = albums.choose(&mut rng);

    // return the album
    match chosen_album {
        Some(album) => album.to_string(),
        None => "No album found".to_string(),
    }
}
