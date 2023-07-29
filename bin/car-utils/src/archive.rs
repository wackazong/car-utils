use crate::error::UtilError;
use blockless_car::utils::archive_local;
use std::path::Path;

#[derive(Debug, clap::Parser)]
pub struct ArchiveCommand {
    #[clap(short, help = "the car file for archive.")]
    car: String,

    #[clap(short, help = "the source directory to be archived.")]
    source: String,
}

impl ArchiveCommand {
    /// archive the local file system to car file
    /// `target` is the car file
    /// `source` is the directory where the archive is prepared.
    pub(crate) fn execute(&self) -> Result<(), UtilError> {
        let file = std::fs::File::create(self.car.as_ref() as &Path).unwrap(); // todo handle error
        archive_local(self.source.as_ref() as &Path, file)?;
        Ok(())
    }
}