pub mod key;
pub mod mouse;
pub mod media;
pub mod dpi;
pub mod keyboard;

use colored::Colorize;
use hidapi::HidDevice;
use crate::args::{ Button, Binding };
use std::{ thread, time::Duration };

const PROFILE_DEFAULT: u8 = 1;

pub fn set(device: &HidDevice, profile: Option<u8>, button: Button, binding: Binding) {
    let mut bfr = [0u8; 65];
    let profile_id = profile.unwrap_or(PROFILE_DEFAULT);

    bfr[3] = 0x02;
    bfr[4] = 0x09;
    bfr[5] = 0x03;
    bfr[7] = profile_id;
    bfr[8] = id_from_btn(button);

    match binding {
        Binding::Key { kind } =>
            key::set(&mut bfr[10..], kind),

        Binding::Mouse(mouse_fn) =>
            mouse::set(&mut bfr[10..], mouse_fn),

        Binding::Keyboard(keyboard_fn) =>
            keyboard::set(&mut bfr[10..], keyboard_fn),

        Binding::Media(media_fn) =>
            media::set(&mut bfr[10..], media_fn),

        Binding::DPI(dpi_fn) =>
            dpi::set(&mut bfr[10..], dpi_fn),

        Binding::None => (),

        _ => println!("(not implemented)"),
    }

    device.send_feature_report(&bfr).unwrap();
    set_and_check(device, &bfr, 0, false);
}

pub fn set_and_check(device: &HidDevice, bfr_w: &[u8], depth: u8, waiting: bool) {
    if depth < 3 {
        if waiting {
            thread::sleep(Duration::from_millis(100));
            set_and_check(device, bfr_w, depth + 1, true);
        }
        else {
            thread::sleep(Duration::from_millis(100));
            let mut bfr_r = [0u8; 55];
            device.get_feature_report(&mut bfr_r).unwrap();
            thread::sleep(Duration::from_millis(40));

            match bfr_r[0] {
                0xA2 => {
                    device.send_feature_report(bfr_w).unwrap();
                    set_and_check(device, bfr_w, depth + 1, false)
                },
                0xA0 => set_and_check(device, bfr_w, depth + 1, false),
                0xA4 => set_and_check(device, bfr_w, depth + 1, true),
                _ => return
            }
        }
    }
    else {
        println!("{}: {}", "Error".bold().red(), "Failed setting key binding!");
    }
}

fn id_from_btn(button: Button) -> u8 {
    match button {
        Button::Left => 1,
        Button::Scroll => 3,
        Button::Right => 2,
        Button::Forward => 5,
        Button::Back => 4,
        Button::DPI => 20,
        Button::ScrollUp => 16,
        Button::ScrollDown => 17,
    }
}
