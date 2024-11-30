use std::fs::read_to_string;
use std::io::{self, ErrorKind};
use std::process::Command;
use std::{
    fs::{read_dir, DirEntry},
    path::{Path, PathBuf},
};

use crate::err::{io_op_call, SError, SResult};

pub fn read_dir_with_single_file(dir: impl AsRef<Path>) -> SResult<PathBuf> {
    let mut files: Vec<io::Result<DirEntry>> = io_op_call(read_dir, &dir)?.collect();
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

pub fn into_not_found_option_or_panic_s<V>(
    path: impl AsRef<Path>,
    result: SResult<V>,
) -> Option<V> {
    match result {
        Ok(v) => Some(v),
        Err(SError::Io { err, .. }) if err.kind() == ErrorKind::NotFound => None,
        Err(err) => panic!("io {} {}", path.as_ref().display(), err),
    }
}

pub fn into_not_found_option_or_panic_io<V>(
    path: impl AsRef<Path>,
    result: io::Result<V>,
) -> Option<V> {
    match result {
        Ok(v) => Some(v),
        Err(err) if err.kind() == ErrorKind::NotFound => {
            println!("[E] not found {}", path.as_ref().display());
            None
        }
        Err(err) => panic!("io {} {}", path.as_ref().display(), err),
    }
}

pub fn execute_command(
    linux_command: &str,
    args: impl IntoIterator<Item = impl AsRef<str>>,
) -> String {
    let mut command = Command::new("/usr/bin/env");

    command.arg(linux_command);
    for arg in args {
        command.arg(arg.as_ref());
    }
    let out = command.output().expect("failed to start command");
    assert!(
        out.status.success(),
        "bad status exit for {}",
        linux_command
    );
    if !out.stderr.is_empty() {
        panic!("stderr: {}", String::from_utf8(out.stderr).unwrap());
    }
    String::from_utf8(out.stdout).unwrap().trim().to_string()
}
