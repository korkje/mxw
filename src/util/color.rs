#[derive(Clone)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

pub fn parse_hex(str: &str) -> Result<Color, &'static str> {
    if str.len() != 6 {
        return Err("Color hex must be of length 6!");
    }

    let parsed = match u32::from_str_radix(str, 16) {
        Ok(value) => value,
        Err(_) => return Err("Could not parse color hex!"),
    };

    let [_, red, green, blue] = parsed.to_be_bytes();

    let color = Color { red, green, blue };

    Ok(color)
}
