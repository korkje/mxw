use std::{ str::FromStr, ops::RangeInclusive };

pub fn in_range(range: &'static RangeInclusive<usize>) -> impl Fn(&str) -> Result<(), String> {
    |str: &str| {
        usize::from_str(str)
            .map(|value| range.contains(&value))
            .map_err(|error| error.to_string())
            .and_then(|result| match result {
                true => Ok(()),
                false => Err(format!("Not in range {}-{}!", range.start(), range.end())),
            })
    }
}
