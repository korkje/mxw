use crate::args::KeyKind;

pub fn set(bfr: &mut [u8], kind: KeyKind) {
    bfr[0] = 0x04;
    bfr[1] = 0x02;

    let (key, modifier) = match kind {
        | KeyKind::ScanCode { key, modifier }
        | KeyKind::KeyCode { key, modifier }
        | KeyKind::Code { key, modifier } => (key, modifier),
    };

    if let Some(value) = modifier {
        bfr[2] = match value.code.as_str() {
            "ControlLeft" => 0x01,
            "ShiftRight" => 0x02,
            "AltRight" => 0x04,
            "MetaLeft" => 0x08,
            _ => 0x00,
        };
    }

    if let Some(value) = key.modifier {
        bfr[2] |= value;
    }
    else {
        bfr[3] = key.scan_code;
    }
}
