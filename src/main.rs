mod cli; // Declare the cli module
mod extract; // Declare the extract module


use extract::extract_zip;
use std::path::Path; // Import Path for handling file paths // Import the extract_zip function from the extract module
fn main() {
    let matches = cli::build_cli().get_matches(); // Parse CLI arguments using the cli module

    let zip_path_str = matches
        .get_one::<String>("zip_path")
        .expect("zip_path argument is required");
    let destination_path_str = matches
        .get_one::<String>("destination_path")
        .expect("destination_path argument is required");

    let zip_path = Path::new(zip_path_str); // Convert the zip_path to a Path
    let destination_path = Path::new(destination_path_str); // Convert the destination_path to a Path

    // Call the extract_zip function and handle any errors
    if let Err(e) = extract_zip(zip_path, destination_path) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
