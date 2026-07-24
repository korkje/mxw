use hidapi::HidDevice;

pub fn set(device: &HidDevice, minutes: u8, seconds: Option<u8>) {
    let mut buffer = [0u8; 65];

    buffer[3] = 0x02;
    buffer[4] = 0x02;
    buffer[6] = 0x07;

    let seconds_total =
        (minutes as u16) * 60 +
        (seconds.unwrap_or(0) as u16);

    if seconds_total > 0 {
        let [first, second] = seconds_total.to_be_bytes();

        buffer[7] = first;
        buffer[8] = second;
    }
    else {
        buffer[7] = 0xFF;
        buffer[8] = 0xFF;
    }

    device.send_feature_report(&buffer).unwrap();
}
