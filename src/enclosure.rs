use std::{
    fs::{read_dir, read_link, read_to_string},
    path::{Path, PathBuf},
};

use crate::err::{io_op, io_op_magic, SError, SResult};

const ENCLOSURE_DIR: &str = "/sys/class/enclosure";

#[derive(Debug)]
pub struct Enclosure {
    enc_id: String,
}

impl Enclosure {
    pub fn load_only() -> SResult<Self> {
        Ok(Self {
            enc_id: Self::find_enclosure_id()?,
        })
    }

    fn find_enclosure_id() -> SResult<String> {
        let read = io_op_magic(read_dir, ENCLOSURE_DIR)?;
        let enclosures: SResult<Vec<String>> = read
            .map(|file| {
                let file = io_op(file, ENCLOSURE_DIR)?;
                SResult::Ok(file.file_name().to_string_lossy().to_string())
            })
            .collect();
        let mut enclosures = enclosures?;

        if enclosures.is_empty() {
            return Err(SError::NoEnclosuresFound);
        }
        if enclosures.len() != 1 {
            return Err(SError::MoreThanOneEnclosureFound);
        }

        let enclosure = enclosures.pop().unwrap();
        Ok(enclosure)
    }

    pub fn slot_len(&self) -> SResult<usize> {
        let slots = io_op_magic(read_to_string, self.files(&["components"]))?;
        Ok(slots.parse().map_err(|_| SError::ComponentsNaN)?)
    }

    pub fn slot(&self, slot_id: usize) -> Slot {
        Slot {
            enc_id: self.enc_id.clone(),
            slot_id,
        }
    }
}

impl HasFiles for Enclosure {
    fn root(&self) -> PathBuf {
        let mut path = PathBuf::from(ENCLOSURE_DIR);
        path.push(&self.enc_id);
        path
    }
}

pub struct Slot {
    enc_id: String,
    slot_id: usize,
}

impl Slot {
    pub fn device(&self) -> Option<PathBuf> {
        let device_file = self.file("device");
        if device_file.is_symlink() {
            let res = io_op_magic(read_link, device_file).unwrap();
            Some(res)
        } else {
            None
        }
    }
}

impl HasFiles for Slot {
    fn root(&self) -> PathBuf {
        let mut path = PathBuf::from(ENCLOSURE_DIR);
        path.push(&self.enc_id);
        path.push(&format!("slot{:02}", self.slot_id));
        path
    }
}

trait HasFiles {
    fn root(&self) -> PathBuf;

    fn files(&self, files: impl IntoIterator<Item = impl AsRef<Path>>) -> PathBuf {
        let mut path = self.root();
        for file in files {
            path.push(file);
        }
        path
    }

    fn file(&self, file: impl AsRef<Path>) -> PathBuf {
        self.files(&[file])
    }
}
