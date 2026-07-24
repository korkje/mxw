use colored::Colorize;
use crate::util::devices::Model;

pub fn get(mouse_model: Model, wired: bool) {
    println!("Model: {}", mouse_model.name.bold());
    println!("Vendor ID: 0x{:04X}", mouse_model.vid);
    println!("Product ID: 0x{:04X}", if wired { mouse_model.pid_wired } else { mouse_model.pid_wireless });
    println!("Connection: {}", if wired { "wired" } else { "wireless" });
}
