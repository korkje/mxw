use colored::Colorize;
use hidapi::HidDevice;
use std::{ thread, time::{ Duration } };

pub fn get(device: &HidDevice, wired: bool) {
    let mut bfr_w = [0u8; 65];

    bfr_w[3] = 0x02;
    bfr_w[4] = 0x02;
    bfr_w[6] = 0x83;

    device.send_feature_report(&bfr_w).unwrap();

    thread::sleep(Duration::from_millis(50));

    let mut bfr_r = [0u8; 65];

    device.get_feature_report(&mut bfr_r).unwrap();

    let mut percentage = bfr_r[8];

    if percentage == 0 {
        percentage = 1;
    }

    let mut status = [0xA1, 0xA4, 0xA2, 0xA0, 0xA3]
        .iter().position(|&s| { s == bfr_r[1] }).unwrap();

    if bfr_r[6] != 0x83 {
        status = 2;
    }

    match (status, wired) {
        (0, false) => println!("{}%", percentage),
        (0, true) => {
            let charging_status = match percentage {
                0..=24 => "charging".red(),
                25..=74 => "charging".yellow(),
                75..=99 => "charging".green(),
                100.. => "fully charged".green().bold(),
            };
            println!("{}% ({})", percentage, charging_status)
        },
        (1, _) => println!("(asleep)"),
        (3, _) => print!("(waking up)"),
        (_, _) => {
            println!(
                "[1:{:0>2X}, 6:{:0>2X}, 8:{:0>2X}] ({})",
                bfr_r[1], bfr_r[6], bfr_r[8], "unknown status".red().bold(),
            );
        },
    }
}
