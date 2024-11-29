use crate::utils::execute_command;

#[derive(Default)]
pub struct ZfsList {
    pub pools: Vec<ZfsListPool>,
}
impl ZfsList {
    pub fn execute() -> Self {
        let res = execute_command(
            "zpool",
            &[
                "list", //
                "-v",   // show underlying vdevs
                "-L",   // use sane block device names
                "-H",   // scripting mode
            ],
        );
        let mut zfslist = ZfsList::default();

        let mut current_pool = ZfsListPool::default();
        for (i, line) in res.split("\n").enumerate() {
            if !line.starts_with("\t") {
                if i != 0 {
                    zfslist.pools.push(current_pool);
                    current_pool = ZfsListPool::default();
                }

                let mut parts = line.split("\t");
                current_pool.pool_name = parts.next().unwrap().to_string();
            } else {
                current_pool.vdevs.push(ZfsListVDev::from_vdev_line(line))
            }
            // println!("{}", line);
        }
        println!(
            "'zpool list' found {} pools ({}) with {} vdevs",
            zfslist.pools.len(),
            zfslist
                .pools
                .iter()
                .map(|pool| pool.pool_name.as_str())
                .collect::<Vec<&str>>()
                .join(", "),
            zfslist
                .pools
                .iter()
                .map(|pool| pool.vdevs.len())
                .sum::<usize>()
        );
        zfslist
    }
}

#[derive(Default)]

pub struct ZfsListPool {
    pub pool_name: String,
    pub vdevs: Vec<ZfsListVDev>,
}

#[derive(Default)]
pub struct ZfsListVDev {
    pub vdev_name: String,
}

impl ZfsListVDev {
    fn from_vdev_line(line: &str) -> Self {
        assert!(line.starts_with("\t"));
        let line = line.trim_start();
        let mut parts = line.split('\t');
        Self {
            vdev_name: parts.next().unwrap().to_string(),
        }
    }
}
