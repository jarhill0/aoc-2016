use std::num::ParseIntError;

pub fn int(string: &str) -> Result<i32, ParseIntError> {
    string.parse::<i32>()
}
