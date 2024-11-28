// #![feature(iter_chain)]

use std::{env::args, path::PathBuf};

use shelf_viewer::{enclosure::Enclosure, err::SResult};

fn main() {
    inner_main().unwrap()
}

fn inner_main() -> SResult<()> {
    println!("start");

    let args: Vec<String> = args().collect();
    let width: usize = args[0].parse().expect("missing width arg");

    let enclosure = Enclosure::load_only()?;
    println!("enclosure {:?}", enclosure);

    let slot_len = enclosure.slot_len()?;
    println!("slot count {}", slot_len);

    let mut states = Vec::with_capacity(slot_len);
    for slot in 0..slot_len {
        let slot = enclosure.slot(slot);

        if let Some(device) = slot.device() {
            states.push(SlotState::Device(device))
        } else {
            states.push(SlotState::Empty)
        }
    }
    print_state(&states, width);

    Ok(())
}

fn print_state(states: &[SlotState], width: usize) {
    let base = "\0xB0";
    let line_sep = base.repeat(width);
    let column_sep = base;
    let cell_width = 10;

    let mut output = String::new();
    for (i, state) in states.iter().enumerate() {
        if i % width == 0 {
            output.push_str(&line_sep);
        }

        match state {
            SlotState::Device(device) => output.push_str(&device.to_string_lossy()),
            SlotState::Empty => output.push_str("___"),
        }

        if i % width == 0 {
            output.push_str(&column_sep);
            output.push('\n');
        }
    }
}

enum SlotState {
    Device(PathBuf),
    Empty,
}
