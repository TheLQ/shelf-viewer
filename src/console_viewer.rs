use crate::colors::{ColorMap, ASCII_RESET};

pub fn print_console_viewer(states: &[SlotState], width: usize) {
    let cell_width = 20;
    let row_sep = "\u{2581}".repeat(cell_width + 1).repeat(width);
    let column_sep = "\u{258F}";

    let mut pool_colors = ColorMap::default();

    let mut output = String::new();
    let slot_order = SlotPrintOrder::BottomLeftGoingUp;
    for (i, slot) in slot_order.order(states.len(), width).iter().enumerate() {
        if i % width == 0 {
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
pub enum SlotState {
    Device { pool: String, device: String },
    Empty,
}

enum SlotPrintOrder {
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
