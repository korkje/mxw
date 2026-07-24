use hidapi::{HidApi, HidDevice};
use colored::Colorize;
use crate::util::fail::Fail;

pub enum Device {
    Real(HidDevice),
    Dry,
}

impl Device {
    pub fn send_feature_report(&self, data: &[u8]) {
        match self {
            Device::Real(device) => {
                device.send_feature_report(data)
                    .map_err(|e| format!("failed to send report to device: {}", e))
                    .or_fail();
            }
            Device::Dry => {
                println!("[dry] send_feature_report ({} bytes):", data.len());
                hexdump(data);
            }
        }
    }

    pub fn get_feature_report(&self, buf: &mut [u8]) {
        match self {
            Device::Real(device) => {
                device.get_feature_report(buf)
                    .map_err(|e| format!("failed to read report from device: {}", e))
                    .or_fail();
            }
            Device::Dry => {
                println!("[dry] get_feature_report ({} bytes): returning zeroed buffer", buf.len());
            }
        }
    }
}

fn hexdump(data: &[u8]) {
    for (row, chunk) in data.chunks(16).enumerate() {
        let mut hex = String::new();
        for (i, byte) in chunk.iter().enumerate() {
            if i == 8 {
                hex.push(' ');
            }
            let cell = format!("{:02X}", byte);
            let cell = if *byte == 0 { cell.dimmed() } else { cell.cyan().bold() };
            hex.push_str(&format!("{} ", cell));
        }
        println!("  {}  {}", format!("{:04X}", row * 16).dimmed(), hex.trim_end());
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Model {
    pub name: &'static str,
    pub vid: u16,
    pub pid_wired: u16,
    pub pid_wireless: u16,
}

pub const DRY_MODEL: Model = Model {
    name: "Dry run (no device)",
    vid: 0x0000,
    pid_wired: 0x0000,
    pid_wireless: 0x0000,
};

pub const SUPPORTED_MICE: &[Model] = &[
    Model {
        name: "Model O Wireless",
        vid: 0x258A,
        pid_wired: 0x2011,
        pid_wireless: 0x2022,
    },
    Model {
        name: "Model O- Wireless",
        vid: 0x258A,
        pid_wired: 0x2013,
        pid_wireless: 0x2024,
    },
    Model {
        name: "Model D Wireless",
        vid: 0x258A,
        pid_wired: 0x2012,
        pid_wireless: 0x2023,
    },
    Model {
        name: "Model D- Wireless",
        vid: 0x258A,
        pid_wired: 0x2014,
        pid_wireless: 0x2025,
    },
];

pub fn find_device(hid_api: &HidApi) -> Result<(HidDevice, Model, bool), String> {
    let (device_info, model, is_wired) = hid_api
        .device_list()
        .filter_map(|d| {
            SUPPORTED_MICE
                .iter()
                .find(|m| {
                    m.vid == d.vendor_id()
                        && d.interface_number() == 0x02
                        && (m.pid_wired == d.product_id() || m.pid_wireless == d.product_id())
                })
                .map(|m| (d, *m, m.pid_wired == d.product_id()))
        })
        .max_by_key(|(_, _, is_wired)| *is_wired)
        .ok_or_else(|| format!(
            "No matching device found!\n\nSupported devices:\n{}",
            supported_mice_list()
        ))?;

    let device = device_info
        .open_device(hid_api)
        .map_err(|error| format!("Failed to open device: {}", error))?;

    Ok((device, model, is_wired))
}

fn supported_mice_list() -> String {
    let width = SUPPORTED_MICE.iter().map(|m| m.name.len()).max().unwrap_or(0);
    SUPPORTED_MICE
        .iter()
        .map(|m| format!(
            "  {:<width$}  VID 0x{:04X}, PID 0x{:04X} (0x{:04X} wired)",
            m.name, m.vid, m.pid_wireless, m.pid_wired, width = width
        ))
        .collect::<Vec<_>>()
        .join("\n")
}
