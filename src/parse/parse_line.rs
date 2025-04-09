use std::str::FromStr;
use regex::Regex;

use crate::board::Line;

use super::{coordinates::{alphabetic_file_to_numeric, rank_to_numeric}, NotationParseError};

impl FromStr for Line
{
    type Err = NotationParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"([a-zA-Z]*)([0-9]*)").unwrap();
        let captures = re.captures(s);
        if captures.is_none()
        {
            return Err(NotationParseError::InvalidFormat(s.to_string()));
        }
        
        let captures = captures.unwrap();
        let file_str = captures.get(1).unwrap().as_str();
        let rank_str = captures.get(2).unwrap().as_str();

        match (file_str.is_empty(), rank_str.is_empty())
        {
            // There should be at least one of the two.
            (true, true) => return Err(NotationParseError::InvalidFormat(s.to_string())),
            // File is blank, so we just have a rank.
            (true, false) => return Ok(Line::Rank(rank_to_numeric(rank_str)?)),
            // Rank is blank, so we just have a file to convert.
            (false, true) => return Ok(Line::File(alphabetic_file_to_numeric(file_str)?)),
            (false, false) => return Ok(Line::RankAndFile(
                    rank_to_numeric(rank_str)?,
                    alphabetic_file_to_numeric(file_str)?
            )),
        }
    }
}
