use crate::logic::calculate;

use gtk::prelude::*;
use gtk::{Button, Grid};
use leptos::{SignalUpdate, WriteSignal};

const GRID_COLUMNS: i32 = 4;
const GRID_SPACING: i32 = 5;
const MAX_OUTPUT_LENGTH: usize = 88;

const BUTTONS: [(&str, i32, i32); 20] = [
    ("CE", 0, 0),
    ("(", 0, 1),
    (")", 0, 2),
    ("^", 0, 3),
    ("7", 1, 0),
    ("8", 1, 1),
    ("9", 1, 2),
    ("-", 1, 3),
    ("4", 2, 0),
    ("5", 2, 1),
    ("6", 2, 2),
    ("÷", 2, 3),
    ("1", 3, 0),
    ("2", 3, 1),
    ("3", 3, 2),
    ("x", 3, 3),
    ("0", 4, 0),
    ("=", 4, 1),
    ("√", 4, 2),
    ("+", 4, 3),
];

const ERROR_MSG: &str = "Malformed input";

fn clear_error_msg(output: &mut Vec<String>) {
    if output.len() == 1 && output[0] == ERROR_MSG {
        output.clear();
    }
}

fn push_digit(output: &mut Vec<String>, digit: String) {
    let len = output.len();
    if let Some(last) = output.last() {
        let last_last_char = last.chars().last().unwrap();
        if len == 1 && (last_last_char == '-' || last_last_char.is_numeric()) {
            output[len - 1].push_str(&digit);
            return;
        } else if len > 1 {
            let second_last_last_char = output[len - 2].chars().last().unwrap();
            if (!second_last_last_char.is_numeric() && last_last_char == '-')
                || last_last_char.is_numeric()
            {
                output[len - 1].push_str(&digit);
                return;
            }
        }
    }
    output.push(digit);
}

pub fn new(set_output: WriteSignal<Vec<String>>, window_width: i32, margin: i32) -> Grid {
    let button_size = (window_width / GRID_COLUMNS) - GRID_SPACING - (margin / GRID_COLUMNS);

    let grid = Grid::builder()
        .column_spacing(GRID_SPACING)
        .row_spacing(GRID_SPACING)
        .margin_top(margin)
        .margin_bottom(margin)
        .margin_start(margin)
        .margin_end(margin)
        .build();

    for (label, row, col) in BUTTONS.iter() {
        let button = Button::builder()
            .label(*label)
            .width_request(button_size)
            .height_request(button_size)
            .build();

        // clear entry button
        if label == &"CE" {
            button.connect_clicked(move |_| {
                set_output.update(|output| {
                    output.pop();
                });
            });
        }
        // equals button
        else if label == &"=" {
            button.connect_clicked(move |_| {
                set_output.update(|output| {
                    let result = calculate(output.to_vec(), ERROR_MSG);
                    output.clear();
                    output.push(result);
                });
            });
        }
        // digit buttons
        else if label.chars().next().unwrap().is_numeric() {
            button.connect_clicked(move |_| {
                set_output.update(|output| {
                    let len = output.join("").len();
                    if len < MAX_OUTPUT_LENGTH {
                        clear_error_msg(output);
                        push_digit(output, label.to_string());
                    }
                });
            });
        }
        // operator buttons
        else {
            button.connect_clicked(move |_| {
                set_output.update(|output| {
                    let len = output.join("").len();
                    if len < MAX_OUTPUT_LENGTH {
                        clear_error_msg(output);
                        output.push(label.to_string());
                    }
                });
            });
        }

        grid.attach(&button, *col, *row, 1, 1);
    }

    grid
}
