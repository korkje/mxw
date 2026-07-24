use crate::args::MediaFn;

pub fn set(bfr: &mut [u8], media_fn: MediaFn) {
    let (b1, b2) = map_from_fn(media_fn);

    bfr[0] = 0x05;
    bfr[1] = 0x02;
    bfr[2] = b1;
    bfr[3] = b2;
}

fn map_from_fn(media_fn: MediaFn) -> (u8, u8) {
    match media_fn {
        MediaFn::Player => (1, 131),
        MediaFn::PlayPause => (0, 205),
        MediaFn::Next => (0, 181),
        MediaFn::Previous => (0, 182),
        MediaFn::Stop => (0, 183),
        MediaFn::Mute => (0, 226),
        MediaFn::VolumeUp => (0, 233),
        MediaFn::VolumeDown => (0, 234),
    }
}
