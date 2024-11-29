use crate::colors::{ColorMap, ASCII_RESET};

pub struct ConsoleViewer {
    pub width: usize,
    pub title: Option<String>,
    pub slot_order: SlotPrintOrder,
}

const U_FULL_BLOCK: &str = "\u{2588}";
const U_LOWER_ONE_EIGHTH_BLOCK: &str = "\u{2581}";
const U_LEFT_ONE_EIGHTH_BLOCK: &str = "\u{258F}";

pub const ALERT_LOCATING: &str = "🚨";

const CELL_WIDTH: usize = 21;

impl ConsoleViewer {
    pub fn print(&self, states: &[SlotState]) {
        let row_sep = U_LOWER_ONE_EIGHTH_BLOCK
            .repeat(CELL_WIDTH + 1/*cols*/ + 3 /*prefix*/ + 2 /*suffix*/)
            .repeat(self.width);
        let row_char_len = row_sep.chars().count();
        let column_sep = U_LEFT_ONE_EIGHTH_BLOCK;

        let mut pool_colors = ColorMap::default();

        let mut output = String::new();

        output.push_str(&U_FULL_BLOCK.repeat(row_char_len));
        output.push('\n');
        if let Some(title) = &self.title {
            output.push_str(&format!("{:-^row_char_len$}", title,));
        }

        for (i, slot) in self
            .slot_order
            .order(states.len(), self.width)
            .iter()
            .enumerate()
        {
            if i % self.width == 0 {
                if i != 0 {
                    output.push_str(&column_sep);
                }
                output.push('\n');
                output.push_str(&row_sep);
                output.push('\n');
            }
            output.push_str(&column_sep);

            if *slot > 23 {
                println!("skippp {}", i);
                continue;
            }
            match &states[*slot] {
                SlotState::Device {
                    group_key,
                    content,

                    prefix,
                    suffix,
                } => {
                    let device_color = pool_colors.get_color(group_key.as_str());

                    output.push_str(&format!(
                        "{}{}{:CELL_WIDTH$}{}{}",
                        device_color,
                        huge_flag_string(prefix),
                        content,
                        wide_flag(suffix),
                        ASCII_RESET
                    ))
                }
                SlotState::Empty { prefix, suffix } => output.push_str(&format!(
                    "{}{:CELL_WIDTH$}{}",
                    huge_flag_string(prefix),
                    "Empty",
                    wide_flag(suffix),
                )),
            }
        }
        output.push('\n');
        output.push_str(&row_sep);

        println!("{}", output);
    }
}

fn wide_flag(flag: &Option<&str>) -> String {
    let flag_char = flag.map(|f| f.to_string()).unwrap_or("  ".to_string());
    flag_char
}

fn wide_flag_string(flag: &Option<String>) -> String {
    let flag_char = flag.clone().unwrap_or("  ".to_string());
    flag_char
}

fn huge_flag(flag: &Option<&str>) -> String {
    let flag_char = flag
        .map(|f| format!("{:>2} ", f))
        .unwrap_or("---".to_string());
    flag_char
}

fn huge_flag_string(flag: &Option<String>) -> String {
    let flag_char = flag
        .clone()
        .map(|f| format!("{:>2} ", f))
        .unwrap_or("---".to_string());
    flag_char
}

fn single_flag<'a>(flag: &Option<&'a str>) -> &'a str {
    let flag_char = flag.unwrap_or("-");
    assert_eq!(flag_char.chars().count(), 1, "too long {}", flag_char);
    flag_char
}

#[derive(PartialEq)]
pub enum SlotState {
    Device {
        group_key: String,
        content: String,
        prefix: Option<String>,
        suffix: Option<&'static str>,
    },
    Empty {
        prefix: Option<String>,
        suffix: Option<&'static str>,
    },
}

pub enum SlotPrintOrder {
    TopLeftGoingDown,
    BottomLeftGoingUp,
}

impl SlotPrintOrder {
    fn order(&self, total_slots: usize, width: usize) -> Vec<usize> {
        assert_eq!(total_slots % width, 0, "invalid width and slot count");
        let height = total_slots / width;
        assert_eq!(height * width, total_slots, "???");
        println!(
            "order {} total {} width {} height {}",
            self.name(),
            total_slots,
            width,
            height
        );

        let mut res = Vec::new();
        match *self {
            SlotPrintOrder::TopLeftGoingDown => {
                for h in 0..height {
                    for w in 0..width {
                        res.push((h) + (w * height));
                    }
                }
            }
            SlotPrintOrder::BottomLeftGoingUp => {
                for h in 0..height {
                    for w in 0..width {
                        res.push((height - h - 1) + (w * height));
                    }
                }
            }
        }
        println!(
            "res {}",
            res.iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(",")
        );
        res
    }

    fn name(&self) -> &str {
        match *self {
            SlotPrintOrder::TopLeftGoingDown => "TopLeftGoingDown",
            SlotPrintOrder::BottomLeftGoingUp => "BottomLeftGoingUp",
        }
    }
}
