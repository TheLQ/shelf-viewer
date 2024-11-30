use std::borrow::Borrow;

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

const PADDING_PREFIX: usize = 4;
const PADDING_SUFFIX: usize = 2;

impl ConsoleViewer {
    pub fn print(&self, states: &[SlotState]) {
        let mut cell_start_width = 0;
        let mut cell_end_width = 0;
        let mut cell_lines = 0;
        for state in states {
            let SlotLabel {
                content_start,
                content_end,
                ..
            } = state.label();

            let lines_width_max = state.lines().iter().map(|v| v.line.len()).max().unwrap();

            cell_start_width = cell_start_width
                .max(content_start.len())
                .max(lines_width_max);
            cell_end_width = cell_end_width.max(content_end.len());
            cell_lines = state.lines().len();
        }
        // anti squish
        cell_start_width += 1;
        cell_end_width += 1;
        let cell_content_width = cell_start_width + cell_end_width;
        println!(
            "{} + {} = {}",
            cell_start_width, cell_end_width, cell_content_width
        );
        // let cell_total_width =  + PADDING_PREFIX + PADDING_SUFFIX;

        let row_sep = U_LOWER_ONE_EIGHTH_BLOCK
            .repeat(cell_content_width + 1 /*cols*/ + PADDING_PREFIX + PADDING_SUFFIX)
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

        let mut slot_line_buffer: Vec<String> = vec!["".to_string(); cell_lines];

        println!("with cell {}", cell_lines);
        for (i, slot) in self
            .slot_order
            .order(states.len(), self.width)
            .iter()
            .enumerate()
        {
            let slot = &states[*slot];

            if i % self.width == 0 {
                if i != 0 {
                    output.push_str(&column_sep);
                }
                output.push('\n');

                if i != 0 {
                    append_lines(slot_line_buffer, &mut output, column_sep);
                }

                slot_line_buffer = vec!["".to_string(); cell_lines];

                output.push_str(&row_sep);
                output.push('\n');
            }
            output.push_str(&column_sep);

            let label_color = if let SlotState::Device(group_key, _, _) = &slot {
                pool_colors.get_color(group_key.as_str())
            } else {
                ""
            };

            let SlotLabel {
                content_start,
                content_end,
                prefix,
                suffix,
            } = slot.label();

            output.push_str(&format!(
                "{}{}{:cell_start_width$}{:>cell_end_width$}{}{}",
                label_color,
                huge_flag(prefix, PADDING_PREFIX),
                content_start,
                content_end,
                huge_flag_str(suffix, PADDING_SUFFIX),
                ASCII_RESET
            ));

            // wheee
            for line_num in 0..cell_lines {
                let line = &mut slot_line_buffer[line_num];
                line.push_str(&column_sep);

                let content = &slot.lines().get(line_num).unwrap().line;

                line.push_str(&format!(
                    "{}{:PADDING_PREFIX$}{:<cell_content_width$}{:PADDING_SUFFIX$}{}",
                    label_color, "", content, "", ASCII_RESET
                ));
            }
        }
        output.push_str(&column_sep);
        output.push('\n');
        append_lines(slot_line_buffer, &mut output, column_sep);

        output.push_str(&row_sep);

        println!("{}", output);
    }
}

fn append_lines(slot_line_buffer: Vec<String>, output: &mut String, column_sep: &str) {
    for line in slot_line_buffer {
        output.push_str(&line);
        output.push_str(column_sep);
        output.push('\n');
    }
}

fn huge_flag(flag: impl Borrow<Option<String>>, padding: usize) -> String {
    let flag_char = flag
        .borrow()
        .as_ref()
        .map(|f| {
            let first_char: char = f.chars().next().unwrap();
            if !first_char.is_ascii_alphanumeric() {
                // emojiis are extra big
                f.to_string()
            } else {
                format!("{:>padding$}", f)
            }
        })
        .unwrap_or(" ".repeat(padding));
    flag_char
}

fn huge_flag_str(flag: &Option<&str>, padding: usize) -> String {
    huge_flag(flag.clone().map(|v| v.to_string()), padding)
}

#[derive(PartialEq)]
pub enum SlotState {
    Device(String, SlotLabel, Vec<SlotLine>),
    Empty(SlotLabel, Vec<SlotLine>),
}

impl SlotState {
    pub fn label(&self) -> &SlotLabel {
        match self {
            Self::Device(_, label, _) => label,
            Self::Empty(label, _) => label,
        }
    }

    pub fn label_mut(&mut self) -> &mut SlotLabel {
        match self {
            Self::Device(_, label, _) => label,
            Self::Empty(label, _) => label,
        }
    }

    pub fn lines(&self) -> &Vec<SlotLine> {
        match self {
            Self::Device(_, _, labels) => labels,
            Self::Empty(_, labels) => labels,
        }
    }

    pub fn lines_mut(&mut self) -> &mut Vec<SlotLine> {
        match self {
            Self::Device(_, _, labels) => labels,
            Self::Empty(_, labels) => labels,
        }
    }
}

#[derive(PartialEq)]

pub struct SlotLabel {
    pub content_start: String,
    pub content_end: String,
    pub prefix: Option<String>,
    pub suffix: Option<&'static str>,
}

#[derive(PartialEq)]
pub struct SlotLine {
    pub line: String,
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
        res
    }

    fn name(&self) -> &str {
        match *self {
            SlotPrintOrder::TopLeftGoingDown => "TopLeftGoingDown",
            SlotPrintOrder::BottomLeftGoingUp => "BottomLeftGoingUp",
        }
    }
}

#[cfg(test)]
mod test {
    use super::SlotPrintOrder;

    #[test]
    fn test_going_down() {
        assert_eq!(
            SlotPrintOrder::TopLeftGoingDown.order(4, 2),
            shelf([
                [0, 2], //
                [1, 3]  //
            ]),
            "4,2"
        );
        assert_eq!(
            SlotPrintOrder::TopLeftGoingDown.order(9, 3),
            shelf([
                [0, 3, 6], //
                [1, 4, 7], //
                [2, 5, 8]  //
            ]),
            "9,3"
        );
    }

    #[test]
    fn test_going_up() {
        assert_eq!(
            SlotPrintOrder::BottomLeftGoingUp.order(4, 2),
            shelf([
                [1, 3], //
                [0, 2]  //
            ]),
            "4,2"
        );
        assert_eq!(
            SlotPrintOrder::BottomLeftGoingUp.order(9, 3),
            shelf([
                [2, 5, 8], //
                [1, 4, 7], //
                [0, 3, 6]  //
            ]),
            "9,3"
        );
    }

    fn shelf<const X: usize, const Y: usize>(order: [[usize; X]; Y]) -> Vec<usize> {
        order.into_iter().flatten().collect()
    }
}
