// #![feature(iter_chain)]

use std::{collections::HashMap, env::args};

use shelf_viewer::{
    colors::{ColorMap, ColorWheel, ASCII_RESET},
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
    print_state(&states, width);

    Ok(())
}

fn print_state(states: &[SlotState], width: usize) {
    let cell_width = 20;
    let row_sep = "\u{2581}".repeat(cell_width + 1).repeat(width);
    let column_sep = "\u{258F}";

    let mut pool_colors = ColorMap::default();

    let mut output = String::new();
    for (i, state) in states.iter().enumerate() {
        if i % width == 0 {
            if i != 0 {
                output.push_str(&column_sep);
            }
            output.push('\n');
            output.push_str(&row_sep);
            output.push('\n');
        }
        output.push_str(&column_sep);

        match state {
            SlotState::Device { device, pool } => {
                let device_color = pool_colors.get_color(pool.clone());

                output.push_str(&format!(
                    "{}{:cell_width$}{}",
                    device_color,
                    format!("{} {}", device, pool),
                    ASCII_RESET
                ))
            }
            SlotState::Empty => output.push_str(&format!("{:cell_width$}", "Empty")),
        }
    }
    output.push('\n');
    output.push_str(&row_sep);

    println!("{}", output);
}

#[derive(PartialEq)]
enum SlotState {
    Device { pool: String, device: String },
    Empty,
}
