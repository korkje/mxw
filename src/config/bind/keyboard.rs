use crate::args::KeyboardFn;

pub fn set(bfr: &mut [u8], keyboard_fn: KeyboardFn) {
    bfr[0] = 0x05;
    bfr[1] = 0x02;
    bfr[2] = id_from_fn(keyboard_fn);
    bfr[3] = 0x0F;
}

fn id_from_fn(keyboard_fn: KeyboardFn) -> u8 {
    match keyboard_fn {
        KeyboardFn::ProfileCycleUp => 1,
        KeyboardFn::ProfileCycleDown => 2,
        KeyboardFn::LayerCycleUp => 3,
        KeyboardFn::LayerCycleDown => 4,
    }
}
