# top-album-shuffle
Shuffle Album is a Rust WebAssembly application that, by default, randomly selects an album from the top 100 all-time Apple Albums. However, you can customize the list of albums by using a CSV file, allowing the app to randomly select from any list of albums you provide.


 This is a small random project I just wanted to do that nothing more ! 

## Features

- Displays a random album from a list of albums.
- Uses the Spotify API to search for albums.
- Provides options to listen to the album on the Spotify web player or open the Spotify app.
- Integration with Spotify OAuth for secure user authentication.

## Technologies Used

- **Rust**: Used for the application logic compiled to WebAssembly.
- **JavaScript**: Used for interacting with the Spotify API and manipulating the DOM.
- **WebAssembly**: To run Rust code in the browser.
- **CSS**: For styling elements and creating visual effects.

## Installation

To run this application locally, follow these steps:

1. **Clone the repository**:
   ```bash
   git clone https://github.com/your-username/shuffle-album.git
    cd shuffle-album
    ```
2. install dependencies 
    ```bash
    cargo install wasm-pack
    wasm-pack build
    ```

3. Start a local server:

    ```bash
    python -m http.server
    ```

Go to http://localhost:8080 (or the port configured by your server) to use the application.





