use std::{
    backtrace::Backtrace,
    fmt::{Debug, Display},
    io,
    path::{Path, PathBuf},
};

use thiserror::Error;

pub type SResult<R> = Result<R, SError>;

#[derive(Error)]
pub enum SError {
    #[error("IoError {path} {err}")]
    Io {
        path: PathBuf,
        err: io::Error,
        bt: Backtrace,
    },
    #[error("NoEnclosuresFound")]
    NoEnclosuresFound,
    #[error("MoreThanOneEnclosureFound")]
    MoreThanOneEnclosureFound,
    #[error("ComponentsNaN {path}")]
    ComponentsNaN { path: PathBuf },
}

impl Debug for SError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl SError {
    pub fn io(path: PathBuf) -> impl FnOnce(io::Error) -> SError {
        // let path = path.as_ref().to_path_buf();
        |err| SError::Io {
            path,
            err,
            bt: Backtrace::capture(),
        }
    }
}

pub fn io_op<T>(source: io::Result<T>, path: impl AsRef<Path>) -> SResult<T> {
    source.map_err(SError::io(path.as_ref().to_path_buf()))
}

pub fn io_op_call<T, S, SP>(source: S, path: SP) -> SResult<T>
where
    S: Fn(SP) -> io::Result<T>,
    SP: AsRef<Path>,
{
    let err_path = path.as_ref().to_path_buf();
    source(path).map_err(SError::io(err_path))
}
