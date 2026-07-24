use crate::util::devices::Device;
use std::{ thread, time::Duration };

pub fn get(device: &Device, wired: bool) {
    let mut bfr_w = [0u8; 65];

    if wired {
        bfr_w[3] = 0x02;
    }

    bfr_w[4] = 0x03;
    bfr_w[6] = 0x81;

    device.send_feature_report(&bfr_w);

    thread::sleep(Duration::from_millis(50));

    let mut bfr_r = [0u8; 65];

    device.get_feature_report(&mut bfr_r);

    println!("{}.{}.{}.{}", bfr_r[7], bfr_r[8], bfr_r[9], bfr_r[10]);
}
