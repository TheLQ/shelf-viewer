// #![feature(iter_chain)]

use std::env::args;

use num_format::ToFormattedString;
use shelf_viewer::{
    console_widget::{
        ConsoleViewer, SlotLabel, SlotLine, SlotPrintOrder, SlotState, ALERT_LOCATING,
    },
    enclosure::Enclosure,
    err::SResult,
    lsblk::Lsblk,
    zfs::ZfsList,
    LOCALE,
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
    let lsblk_list = Lsblk::execute();

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
            let line = slot.device_wwid().unwrap_or(not_found("no_wwid"));
            slot_state.lines_mut().push(SlotLine { line });
        }

        if is_wwid {
            let line = slot
                .device_wwid()
                .unwrap_or(not_found("no_wid_file"))
                .replace("naa.", "wwn-0x");
            slot_state.lines_mut().push(SlotLine { line });
        }

        // Always ATA
        // if is_wwid {
        //     let line = slot.device_vendor().unwrap_or(not_found("no_vendor_file"));
        //     slot_state.lines_mut().push(SlotLine { line });
        // }

        if is_wwid {
            let line = slot.device_model().unwrap_or(not_found("no_model_file"));
            slot_state.lines_mut().push(SlotLine { line });
        }

        if is_wwid {
            let entry = slot
                .block_name()
                .map(|block_name| lsblk_list.iter().find(|v| v.device == block_name))
                .flatten();
            let line = match entry {
                Some(entry) => {
                    let bytes: usize = entry.bytes.parse().unwrap();
                    const GIGABYTE: usize = 1000usize.pow(3);

                    let quantity = (bytes / GIGABYTE).to_formatted_string(LOCALE);
                    format!("{} G", quantity)
                }
                None => not_found("no_lsblk"),
            };
            slot_state.lines_mut().push(SlotLine { line });
            // todo: this is some percent off
            // let line;
            // if let Some(bytes_str) = slot.block_size() {
            //     let bytes: usize = bytes_str.parse().expect(&bytes_str);
            //     // 512 even on 4kn drives
            //     let disk_block_size: usize = 512;
            //     const GIGABYTE: usize = 1024usize.pow(3);
            //     let quantity = (bytes * disk_block_size / GIGABYTE).to_formatted_string(LOCALE);
            //     line = format!("{} G", quantity)
            // } else {
            //     line = "no_size_file".into();
            // }
        }

        states.push(slot_state);
    }

    let presentation_mode = true;
    if presentation_mode {
        for slot_state in &mut states {
            for line in slot_state.lines_mut() {
                if line.line.ends_with(" G") {
                    continue;
                }

                const BLANKING: usize = 8;
                let line_len = line.line.len();
                let blanking = BLANKING.min(line_len);
                line.line
                    .replace_range((line_len - blanking)..(line_len), &"0".repeat(blanking));
                println!("{}", line.line);
            }
        }

        let mut bad_pools: Vec<String> = Vec::new();
        for state in &mut states {
            let bad_name = &mut state.label_mut().content_start;
            if !bad_name.starts_with("ZFS") {
                continue;
            }
            let safe_pos = match bad_pools.iter().position(|p| p == bad_name) {
                Some(p) => p,
                None => {
                    bad_pools.push(bad_name.clone());
                    bad_pools.len() - 1
                }
            };
            *bad_name = format!("ZFS pool{}", safe_pos);
        }

        let mut labels_to_edit: Vec<&SlotLabel> = states.iter().map(|e| e.label()).collect();
        labels_to_edit.sort_by_key(|l| &l.content_start);
        labels_to_edit.dedup();
    }

    let title = format!(
        "{} {} - {}",
        enclosure.device_vendor().unwrap_or(not_found("no_vendor")),
        enclosure.device_model().unwrap_or(not_found("no_model")),
        enclosure.enc_id()
    );

    ConsoleViewer {
        title: Some(title),
        width,
        slot_order: SlotPrintOrder::BottomLeftGoingUp,
    }
    .print(&states);

    Ok(())
}

const ENABLE_NOT_FOUND: bool = false;

fn not_found(msg: &str) -> String {
    if ENABLE_NOT_FOUND {
        msg.into()
    } else {
        "".into()
    }
}
