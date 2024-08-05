use clap::{Arg, Command};

// Function to build the CLI command structure
pub fn build_cli() -> Command {
    Command::new("zip_extractor") // Create a new command named "zip_extractor"
        .version("1.0.0") // Set the version of the command
        .about("Extracts a zip file to a specified destination") //Description of the command
        .arg(
            Arg::new("zip_path") // Define a new argument named "zip_path"
                .required(true) // Make this argument required
                .index(1), // Position of the argument in the cli
        )
        .arg(Arg::new("destination_path").required(true).index(2))
}
