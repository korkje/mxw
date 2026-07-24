use hidapi::{HidApi, HidDevice};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Model {
    pub name: &'static str,
    pub vid: u16,
    pub pid_wired: u16,
    pub pid_wireless: u16,
}

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
        pid_wired: 0x2024,
        pid_wireless: 0x2013,
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

pub fn find_device(hid_api: &HidApi) -> Option<(HidDevice, Model, bool)> {
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
        .max_by_key(|(_, _, is_wired)| *is_wired)?;

    let device = device_info.open_device(hid_api).ok()?;

    Some((device, model, is_wired))
}

pub fn supported_mice_list() -> String {
    SUPPORTED_MICE
        .iter()
        .map(|m| format!(
            "{} (vendor: 0x{:04X}, wired: 0x{:04X}, wireless: 0x{:04X})",
            m.name, m.vid, m.pid_wired, m.pid_wireless
        ))
        .collect::<Vec<_>>()
        .join(", ")
}
