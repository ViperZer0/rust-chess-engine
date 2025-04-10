use std::str::FromStr;

use regex::Regex;

use crate::board::Square;

use super::{coordinates::{alphabetic_file_to_numeric, rank_to_numeric}, NotationParseError};

impl FromStr for Square
{
    type Err = NotationParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"([a-zA-Z]+)([0-9]+)").unwrap();
        let captures = re.captures(s);
        if captures.is_none()
        {
            return Err(NotationParseError::InvalidFormat(s.to_string()));
        }

        let captures = captures.unwrap();
        let file_str = captures.get(1).unwrap().as_str();
        let rank_str = captures.get(2).unwrap().as_str();
        let file = alphabetic_file_to_numeric(file_str)?;
        let rank = rank_to_numeric(rank_str)?;

        Ok(Square::new(rank, file))
    }
}
