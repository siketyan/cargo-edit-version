mod manifest;

use toml_edit::value;

use std::env::args;

use crate::manifest::Manifest;

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("Failed to read from or write to the manifest file: {0}")]
    Manifest(#[from] crate::manifest::Error),
}

type Result<T> = std::result::Result<T, Error>;

fn handle(version: &str) -> Result<()> {
    let path = "./Cargo.toml";
    let mut manifest = Manifest::open(path).map_err(Error::Manifest)?;
    let mut document = manifest.read_document().map_err(Error::Manifest)?;

    document["package"]["version"] = value(version);

    manifest.write_document(document).map_err(Error::Manifest)
}

fn main() {
    let arguments = args().collect::<Vec<String>>();
    if arguments.len() < 2 {
        eprintln!("Usage: {} [version]", &arguments[0]);
        std::process::exit(1);
    }

    if let Result::Err(error) = handle(&arguments[1]) {
        eprintln!("Error: {}", error);
    }
}
