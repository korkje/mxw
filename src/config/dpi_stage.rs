use crate::util::devices::Device;

pub fn set(device: &Device, profile: u8, id: u8) {
    let mut bfr = [0u8; 65];

    bfr[3] = 0x02;
    bfr[4] = 0x02;
    bfr[5] = 0x01;
    bfr[6] = 0x02;
    bfr[7] = profile;
    bfr[8] = id;

    device.send_feature_report(&bfr).unwrap();
}
