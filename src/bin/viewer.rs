// #![feature(iter_chain)]

use std::env::args;

use shelf_viewer::{
    console_widget::{ConsoleViewer, SlotPrintOrder, SlotState, ALERT_LOCATING},
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
    let slot_len = enclosure.slot_len().unwrap();
    println!("enclosure {:?} with {} slots", enclosure, slot_len);

    let zfs_list = ZfsList::execute();

    let mut states = Vec::with_capacity(slot_len);
    for slot_id in 0..slot_len {
        let slot = enclosure.slot(slot_id);

        let mut device_flag = None;
        if slot.is_locating() {
            device_flag = Some(ALERT_LOCATING);
        }

        if let Some(device) = slot.block_device_name() {
            let mut message = None;
            for pool in &zfs_list.pools {
                for vdev in &pool.vdevs {
                    if vdev.vdev_name == device {
                        message = Some(SlotState::Device {
                            group_key: format!("ZFS {}", pool.pool_name),
                            content: format!("ZFS {} - {}", pool.pool_name, vdev.vdev_name),
                            end_flag_char: device_flag,
                        });
                    }
                }
            }
            if message == None {
                message = Some(SlotState::Device {
                    group_key: "___".to_string(),
                    content: format!("___ {}", device),
                    end_flag_char: device_flag,
                })
            }
            states.push(message.unwrap())
        } else {
            states.push(SlotState::Empty {
                end_flag_char: device_flag,
            })
        }
    }
    println!("states {}", states.len());

    let title = format!(
        "{} {}",
        enclosure.device_vendor().unwrap_or("no_vendor".into()),
        enclosure.device_model().unwrap_or("no_model".into())
    );

    ConsoleViewer {
        title: Some(title),
        width,
        slot_order: SlotPrintOrder::BottomLeftGoingUp,
    }
    .print(&states);

    Ok(())
}
