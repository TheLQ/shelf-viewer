// #![feature(iter_chain)]

use std::env::args;

use shelf_viewer::{
    console_widget::{
        ConsoleViewer, SlotLabel, SlotLine, SlotPrintOrder, SlotState, ALERT_LOCATING,
    },
    enclosure::{Enclosure, Slot},
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

        let prefix = Some(format!("{} ", slot_id.to_string()));

        let mut slot_state;
        if let Some(device) = slot.block_name() {
            slot_state = SlotState::Device(
                "__".to_string(),
                SlotLabel {
                    content_start: "__".to_string(),
                    content_end: device.clone(),
                    prefix,
                    suffix: device_flag,
                },
                Vec::new(),
            );
            'outer: for pool in &zfs_list.pools {
                for vdev in &pool.vdevs {
                    if vdev.vdev_name == device {
                        let key = format!("ZFS {}", pool.pool_name);
                        if let SlotState::Device(group_key, SlotLabel { content_start, .. }, _) =
                            &mut slot_state
                        {
                            *group_key = key.clone();
                            *content_start = key;
                        }
                        break 'outer;
                    }
                }
            }
        } else {
            slot_state = SlotState::Empty(
                SlotLabel {
                    content_start: "".to_string(),
                    content_end: "Empty".to_string(),
                    prefix: prefix.clone(),
                    suffix: device_flag,
                },
                Vec::new(),
            )
        }

        let is_wwid = true;

        if is_wwid {
            let line = slot.device_wwid().unwrap_or("no_wwid".to_string());
            slot_state.lines_mut().push(SlotLine { line });
        }

        if is_wwid {
            let line = slot
                .device_wwid()
                .unwrap_or("no_wid_file".to_string())
                .replace("naa.", "wwn-0x");
            slot_state.lines_mut().push(SlotLine { line });
        }

        if is_wwid {
            let line = slot
                .device_vendor()
                .unwrap_or("no_vendor_file".to_string())
                .replace("naa.", "wwn-0x");
            slot_state.lines_mut().push(SlotLine { line });
        }

        if is_wwid {
            let line = slot
                .device_model()
                .unwrap_or("no_model_file".to_string())
                .replace("naa.", "wwn-0x");
            slot_state.lines_mut().push(SlotLine { line });
        }

        states.push(slot_state);
    }

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
