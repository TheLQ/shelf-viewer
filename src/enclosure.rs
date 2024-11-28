use std::{
    fs::{read_dir, read_to_string},
    io::{self, ErrorKind},
    path::{Path, PathBuf},
};

use crate::{
    err::{io_op, io_op_magic, SError, SResult},
    utils::{read_dir_with_single_file, read_to_string_trim},
};

const ENCLOSURE_DIR: &str = "/sys/class/enclosure";

#[derive(Debug)]
pub struct Enclosure {
    enc_id: String,
}

impl Enclosure {
    pub fn load_only() -> SResult<Self> {
        Ok(Self {
            enc_id: Self::find_only_enclosure_id()?,
        })
    }

    fn find_only_enclosure_id() -> SResult<String> {
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
        let components_path = self.files(&["components"]);
        let slots = io_op_magic(read_to_string, &components_path)?;
        Ok(slots.trim().parse().map_err(|_| SError::ComponentsNaN {
            path: components_path,
        })?)
    }

    pub fn slot(&self, slot_id: usize) -> Slot {
        Slot {
            enc_id: self.enc_id.clone(),
            slot_id,
        }
    }

    pub fn device_vendor(&self) -> SResult<String> {
        let path = self.files(&["device", "vendor"]);
        io_op_magic(read_to_string_trim, &path)
    }

    pub fn device_model(&self) -> SResult<String> {
        let path = self.files(&["device", "model"]);
        io_op_magic(read_to_string_trim, &path)
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
    pub fn block_device_sys(&self) -> Option<PathBuf> {
        let block_root_dir = self.files(&["device", "block"]);
        match read_dir_with_single_file(&block_root_dir) {
            Ok(block_dir) => Some(block_dir),
            Err(SError::Io { err, .. }) if err.kind() == ErrorKind::NotFound => None,
            Err(e) => panic!("fail block device {} {}", block_root_dir.display(), e),
        }
    }

    pub fn block_device_name(&self) -> Option<String> {
        self.block_device_sys().map(|block_device_sys| {
            let os_name = block_device_sys.file_name().unwrap();
            os_name.to_str().unwrap().to_string()
        })
    }

    pub fn is_locating(&self) -> bool {
        let path: PathBuf = self.file("locate");
        match read_to_string(&path) {
            Ok(content) => content.trim() == "1",
            // enclosure doesn't support locating
            Err(e) if e.kind() == ErrorKind::NotFound => false,
            Err(e) => panic!("fail readling locate {} {}", path.display(), e),
        }
    }
}

impl HasFiles for Slot {
    fn root(&self) -> PathBuf {
        let mut path = PathBuf::from(ENCLOSURE_DIR);
        path.push(&self.enc_id);
        path.push(&format!("Slot{:02}", self.slot_id));
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
