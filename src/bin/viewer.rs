// #![feature(iter_chain)]

use std::env::args;

use shelf_viewer::{
    console_viewer::{print_console_viewer, SlotState},
    enclosure::Enclosure,
    err::SResult,
    zfs::ZfsList,
};

fn main() {
    inner_main().unwrap()
}

fn inner_main() -> SResult<()> {
    println!("start");

    let args: Vec<String> = args().collect();
    println!("args {}", args.join(","));
    let width: usize = args[1].parse().expect("missing width arg");

    let enclosure = Enclosure::load_only()?;
    println!("enclosure {:?}", enclosure);

    let slot_len = enclosure.slot_len()?;
    println!("slot count {}", slot_len);

    let zfs_list = ZfsList::execute();

    let mut states = Vec::with_capacity(slot_len);
    for slot in 0..slot_len {
        let slot = enclosure.slot(slot);

        if let Some(device) = slot.block_device_name() {
            let mut message = None;
            for pool in &zfs_list.pools {
                for vdev in &pool.vdevs {
                    if vdev.vdev_name == device {
                        message = Some(SlotState::Device {
                            pool: format!("ZFS {}", pool.pool_name),
                            device: format!("{}", vdev.vdev_name),
                        });
                    }
                }
            }
            if message == None {
                message = Some(SlotState::Device {
                    pool: "___".to_string(),
                    device,
                })
            }
            states.push(message.unwrap())
        } else {
            states.push(SlotState::Empty)
        }
    }
    println!("states {}", states.len());
    print_console_viewer(&states, width);

    Ok(())
}
