use hidapi::HidDevice;

pub fn set(device: &HidDevice, profile: u8, ms: u8) {
    let mut bfr = [0u8; 65];

    bfr[3] = 0x02;
    bfr[4] = 0x01;
    bfr[6] = 0x08;
    bfr[7] = profile;
    bfr[8] = ms;

    device.send_feature_report(&bfr).unwrap();
}
