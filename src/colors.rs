use std::{collections::HashMap, hash::Hash};

const BACKGROUND_COLORS: [&str; 9] = [
    "\x1b[38;5;22m", //
    "\x1b[38;5;58m", //
    "\x1b[38;5;94m", //
    //
    "\x1b[38;5;24m", //
    "\x1b[38;5;60m", //
    "\x1b[38;5;96m", //
    //
    "\x1b[38;5;27m", //
    "\x1b[38;5;63m", //
    "\x1b[38;5;99m", //
];

pub const ASCII_RESET: &str = "\x1b[0m";

#[derive(Default)]
pub struct ColorWheel {
    i: usize,
}

impl ColorWheel {
    pub fn next(&mut self) -> &'static str {
        let color = BACKGROUND_COLORS[self.i];
        self.i += 1;
        color
    }
}

#[derive(Default)]
pub struct ColorMap<T> {
    data: HashMap<T, &'static str>,
    color_wheel: ColorWheel,
}

impl<T: Eq + Hash + ToString> ColorMap<T> {
    pub fn get_color(&mut self, data_entry: T) -> &'static str {
        let dc_entry = data_entry.to_string();
        self.data.entry(data_entry).or_insert_with(|| {
            println!("inserting {}", dc_entry);
            self.color_wheel.next()
        })
    }
}
