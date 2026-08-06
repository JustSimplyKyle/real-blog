use std::error::Error as StdError;
use std::fmt;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub use OverwriteBehavior::{AllowOverwrite, DisallowOverwrite};

#[derive(Clone, Copy)]
pub enum OverwriteBehavior {
    AllowOverwrite,
    DisallowOverwrite,
}

#[derive(Debug)]
pub enum Error<E> {
    Internal(io::Error),
    User(E),
}

impl From<Error<io::Error>> for io::Error {
    fn from(error: Error<io::Error>) -> Self {
        match error {
            Error::Internal(error) | Error::User(error) => error,
        }
    }
}

impl<E: fmt::Display> fmt::Display for Error<E> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Internal(error) => error.fmt(formatter),
            Error::User(error) => error.fmt(formatter),
        }
    }
}

impl<E: StdError + 'static> StdError for Error<E> {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::Internal(error) => Some(error),
            Error::User(error) => Some(error),
        }
    }
}

pub struct AtomicFile {
    path: PathBuf,
    #[cfg_attr(target_arch = "wasm32", allow(dead_code))]
    overwrite: OverwriteBehavior,
}

impl AtomicFile {
    pub fn new(path: impl AsRef<Path>, overwrite: OverwriteBehavior) -> Self {
        Self {
            path: path.as_ref().to_owned(),
            overwrite,
        }
    }

    pub fn new_with_tmpdir(
        path: impl AsRef<Path>,
        overwrite: OverwriteBehavior,
        _tmpdir: impl AsRef<Path>,
    ) -> Self {
        Self::new(path, overwrite)
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn write<T, E>(
        &self,
        operation: impl FnOnce(&mut fs::File) -> Result<T, E>,
    ) -> Result<T, Error<E>> {
        self.write_with_options(operation, fs::OpenOptions::new())
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn write_with_options<T, E>(
        &self,
        operation: impl FnOnce(&mut fs::File) -> Result<T, E>,
        mut options: fs::OpenOptions,
    ) -> Result<T, Error<E>> {
        let temporary = self.path.with_extension("atomicwrite.tmp");
        options.write(true).create(true).truncate(true);
        let mut file = options.open(&temporary).map_err(Error::Internal)?;
        let result = operation(&mut file).map_err(Error::User)?;
        file.sync_all().map_err(Error::Internal)?;
        drop(file);

        if matches!(self.overwrite, DisallowOverwrite) && self.path.exists() {
            return Err(Error::Internal(io::Error::new(
                io::ErrorKind::AlreadyExists,
                "destination already exists",
            )));
        }

        fs::rename(temporary, &self.path).map_err(Error::Internal)?;
        Ok(result)
    }

    #[cfg(target_arch = "wasm32")]
    pub fn write_with_options<T, E>(
        &self,
        _operation: impl FnOnce(&mut fs::File) -> Result<T, E>,
        _options: fs::OpenOptions,
    ) -> Result<T, Error<E>> {
        Err(Error::Internal(io::Error::new(
            io::ErrorKind::Unsupported,
            "atomic filesystem writes are unavailable in a browser",
        )))
    }
}
