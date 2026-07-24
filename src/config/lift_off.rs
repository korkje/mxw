use hidapi::HidDevice;

pub fn set(device: &HidDevice, mm: u8) {
    let mut bfr = [0u8; 65];

    bfr[3] = 0x02;
    bfr[4] = 0x01;
    bfr[5] = 0x01;
    bfr[6] = 0x07;
    bfr[7] = mm - 1;

    device.send_feature_report(&bfr).unwrap();
}
