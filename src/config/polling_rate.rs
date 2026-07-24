use hidapi::HidDevice;

pub fn set(device: &HidDevice, ms: u8) {
    let mut bfr = [0u8; 65];

    bfr[3] = 0x02;
    bfr[4] = 0x01;
    bfr[5] = 0x01;
    bfr[7] = ms;

    device.send_feature_report(&bfr).unwrap();
}
