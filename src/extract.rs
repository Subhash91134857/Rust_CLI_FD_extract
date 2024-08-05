use std::fs::File; // Import File for file operations
use std::io::{self, BufReader}; // Import io for input/Output operations and BufReader for buffered reading
use std::path::Path; // Import Path for handling file paths
use zip::read::ZipArchive; // Import ZipArchives for reading zip files

// Function to extract a zip file to a specified destination
pub fn extract_zip(zip_path: &Path, destination: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // Attempt to open the zip file, with error handling
    let file = File::open(zip_path).map_err(|e| format!("Failed to open zip file: {}", e))?;

    // Create a ZipArchive from the file
    let mut archive = ZipArchive::new(BufReader::new(file))
        .map_err(|e| format!("Failed to read zip file: {}", e))?;

    // Iterate over each file in the zip archive

    for i in 0..archive.len() {
        let mut file = archive
            .by_index(i)
            .map_err(|e| format!("Failed to access file in archive: {}", e))?;

        let outpath = destination.join(file.name());

        if (&*file.name()).ends_with('/') {
            // Create the directory

            std::fs::create_dir_all(&outpath)
                .map_err(|e| format!("Failed to create directory: {}", e))?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    // Create the parent directory if it doesn't exist
                    std::fs::create_dir_all(&p)
                        .map_err(|e| format!("Failed to create parent directory: {}", e))?;
                }
            }

            // Create the output file

            let mut outfile = File::create(&outpath)
                .map_err(|e| format!("Failed to create output file: {}", e))?;

            // Copy the file contents
            io::copy(&mut file, &mut outfile)
                .map_err(|e| format!("Failed to copy file contents: {}", e))?;
        }

        // Preserve permissions on Unix systems
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Some(mode) = file.unix_mode() {
                std::fs::set_permissions(&outpath, std::fs::Permissions::from_mode(mode))
                    .map_err(|e| format!("Failed to set permissions: {}", e))?;
            }
        }
    }

    Ok(())
}
