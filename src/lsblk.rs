use crate::utils::execute_command;

pub struct Lsblk {}

impl Lsblk {
    pub fn execute() -> Vec<LsblkEntry> {
        let res = execute_command(
            "lsblk",
            &[
                "-b", // byte sizes
                "-d", // devices only not partitions
                "-r", // scripting mode
                "-n", // no headers
                "-o", //
                "name,size",
            ],
        );
        let mut entries = Vec::new();
        for line in res.split("\n") {
            let (device, bytes) = line.split_at(line.chars().position(|c| c == ' ').unwrap());
            entries.push(LsblkEntry {
                device: device.to_string(),
                bytes: bytes[1..].to_string(),
            });
        }

        entries
    }
}

pub struct LsblkEntry {
    pub device: String,
    pub bytes: String,
}
