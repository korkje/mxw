use colored::Colorize;
use crate::util::devices::Model;

pub fn get(mouse_model: Model, wired: bool) {
    println!("Model: {}", mouse_model.name.bold());
    println!("VID: 0x{:04X}", mouse_model.vid);
    println!("PID: 0x{:04X}", if wired { mouse_model.pid_wired } else { mouse_model.pid_wireless });
    println!("Connection: {}", if wired { "wired" } else { "wireless" });
}
