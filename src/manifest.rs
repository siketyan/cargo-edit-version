use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use toml_edit::{Document, TomlError};

#[derive(Debug, thiserror::Error)]
pub(crate) enum Error {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Toml error: {0}")]
    Toml(#[from] TomlError),
}

type Result<T> = std::result::Result<T, Error>;

pub(crate) struct Manifest {
    file: File,
}

impl Manifest {
    fn new(file: File) -> Self {
        Self { file }
    }

    pub(crate) fn open(path: &str) -> Result<Self> {
        Ok(Self::new(
            OpenOptions::new()
                .read(true)
                .write(true)
                .append(false)
                .open(path)
                .map_err(Error::Io)?,
        ))
    }

    pub(crate) fn read_document(&mut self) -> Result<Document> {
        let mut toml = String::new();

        self.file.read_to_string(&mut toml).map_err(Error::Io)?;
        toml.parse().map_err(Error::Toml)
    }

    pub(crate) fn write_document(&mut self, document: Document) -> Result<()> {
        let toml = document.to_string();
        let bytes = toml.as_bytes();

        self.write(bytes).map_err(Error::Io)
    }

    fn write(&mut self, bytes: &[u8]) -> std::io::Result<()> {
        let mut file = &self.file;

        file.seek(SeekFrom::Start(0))?;
        file.write_all(bytes)?;
        file.set_len(bytes.len() as u64)?;
        file.flush()
    }
}
