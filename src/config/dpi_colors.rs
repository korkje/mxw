use hidapi::HidDevice;
use crate::util::color::Color;

const PROFILE_DEFAULT: u8 = 1;

pub fn set(device: &HidDevice, profile: Option<u8>, colors: Vec<Color>) {
    let mut bfr = [0u8; 65];
    let profile_id = profile.unwrap_or(PROFILE_DEFAULT);

    bfr[3] = 0x02;
    bfr[4] = 0x13;
    bfr[5] = 0x02;
    bfr[6] = 0x01;
    bfr[7] = profile_id;

    for i in 0..colors.len() {
        bfr[8 + 3 * i + 0] = colors[i].red;
        bfr[8 + 3 * i + 1] = colors[i].green;
        bfr[8 + 3 * i + 2] = colors[i].blue;
    }

    device.send_feature_report(&bfr).unwrap();
}
