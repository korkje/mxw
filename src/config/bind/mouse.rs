use crate::args::MouseFn;

pub fn set(bfr: &mut [u8], mouse_fn: MouseFn) {
    let id = id_from_fn(mouse_fn);

    bfr[0] = 0x01;
    bfr[1] = 0x01;
    bfr[2] = id;

    if id == 12 {
        bfr[0] = 12;
        bfr[2] = 1;
    }
    else if id == 24 {
        bfr[0] = 8;
        bfr[2] = 4;
    }
    else if id == 25 {
        bfr[0] = 8;
        bfr[2] = 3;
    }
}

fn id_from_fn(mouse_fn: MouseFn) -> u8 {
    match mouse_fn {
        MouseFn::Left => 1,
        MouseFn::Scroll => 3,
        MouseFn::Right => 2,
        MouseFn::Forward => 5,
        MouseFn::Back => 4,
        MouseFn::ScrollUp => 16,
        MouseFn::ScrollDown => 17,
        MouseFn::ProfileCycleUp => 24,
        MouseFn::ProfileCycleDown => 25,
        MouseFn::BatteryStatus => 12,
    }
}
