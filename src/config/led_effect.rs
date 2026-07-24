use std::process;

use colored::Colorize;
use hidapi::HidDevice;
use crate::args::Effect;

pub fn set(device: &HidDevice, profile: u8, effect: Effect) {
    let mut bfr = [0u8; 65];

    bfr[3] = 0x02;
    bfr[5] = 0x02;
    bfr[7] = profile;
    bfr[8] = 0xFF;

    match effect {
        Effect::Glorious { rate } => {
            bfr[4] = 0x05;
            bfr[9] = 0x01;
            bfr[11] = rate_check(rate, 1);
        },

        Effect::Cycle { rate } => {
            bfr[4] = 0x05;
            bfr[9] = 0x02;
            bfr[11] = rate_check(rate, 2);
            bfr[12] = 0xFF;
        },

        Effect::Pulse { rate, colors } => {
            bfr[4] = (colors.len() as u8) * 3 + 5;
            bfr[9] = 0x03;
            bfr[11] = rate_check(rate, 3);

            for i in 0..6 {
                if i >= colors.len() {
                    bfr[12 + 3 * i + 0] = 0x00;
                    bfr[12 + 3 * i + 1] = 0x00;
                    bfr[12 + 3 * i + 2] = 0x00;
                }
                else {
                    bfr[12 + 3 * i + 0] = colors[i].red;
                    bfr[12 + 3 * i + 1] = colors[i].green;
                    bfr[12 + 3 * i + 2] = colors[i].blue;
                }
            }
        },

        Effect::Solid { color } => {
            bfr[4] = 0x08;
            bfr[9] = 0x04;

            bfr[12 + 0] = color.red;
            bfr[12 + 1] = color.green;
            bfr[12 + 2] = color.blue;
        },

        Effect::PulseOne { rate, color } => {
            bfr[4] = 0x08;
            bfr[9] = 0x05;
            bfr[11] = rate_check(rate, 5);

            bfr[12 + 0] = color.red;
            bfr[12 + 1] = color.green;
            bfr[12 + 2] = color.blue;
        }

        Effect::Tail { rate } => {
            bfr[4] = 0x05;
            bfr[9] = 0x06;
            bfr[11] = rate_check(rate, 6);
        },

        Effect::Rave { rate, colors } => {
            bfr[4] = (colors.len() as u8) * 3 + 5;
            bfr[9] = 0x07;
            bfr[11] = rate_check(rate, 7);

            for i in 0..2 {
                if i >= colors.len() {
                    bfr[12 + 3 * i + 0] = 0x00;
                    bfr[12 + 3 * i + 1] = 0x00;
                    bfr[12 + 3 * i + 2] = 0x00;
                }
                else {
                    bfr[12 + 3 * i + 0] = colors[i].red;
                    bfr[12 + 3 * i + 1] = colors[i].green;
                    bfr[12 + 3 * i + 2] = colors[i].blue;
                }
            }
        },

        Effect::Wave { rate } => {
            bfr[4] = 0x05;
            bfr[9] = 0x08;
            bfr[11] = rate_check(rate, 8);
        },

        Effect::Off => {
            bfr[4] = 0x05;
            bfr[9] = 0x00;
        },
    }

    device.send_feature_report(&bfr).unwrap();
}

const RATE_DEFAULT: u8 = 40;

fn rate_check(rate: Option<u8>, effect_id: u8) -> u8 {

    let rate_unwrapped = rate.unwrap_or(RATE_DEFAULT);

    let rate_checked = match rate_unwrapped {
        0..=100 => rate_unwrapped,
        _ => {
            println!("{}: {}", "Error".bold().red(), "Rate must be 0-100!");
            process::exit(1);
        },
    };

    match effect_id {
        7 | 8 => (105 - rate_checked) * 2,
        _ => (105 - rate_checked) / 5
    }
}
