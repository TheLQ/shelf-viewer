use std::fs::read_to_string;
use std::io;
use std::{
    fs::{read_dir, DirEntry},
    path::{Path, PathBuf},
};

use crate::err::{io_op_magic, SResult};

pub fn read_dir_with_single_file(dir: impl AsRef<Path>) -> SResult<PathBuf> {
    let mut files: Vec<io::Result<DirEntry>> = io_op_magic(read_dir, &dir)?.collect();
    assert_eq!(
        files.len(),
        1,
        "Unexpected block dir {}",
        dir.as_ref().display()
    );
    let file = files.pop().unwrap().unwrap();
    Ok(file.path())
}

pub fn read_to_string_trim(path: impl AsRef<Path>) -> io::Result<String> {
    Ok(read_to_string(path)?.trim().to_string())
}
