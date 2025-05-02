use std::ops::Not;

/// The two side colors, white and black.
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum PlayerColor
{
    /// The white player color
    White,
    /// The black player color
    Black,
}

impl Default for PlayerColor
{
    /// The default starting color is white.
    fn default() -> Self {
        PlayerColor::White
    }
}

impl Not for PlayerColor
{
    type Output = PlayerColor;

    /// Returns the opposite color of the one provided.
    ///
    /// # Examples
    ///
    /// ```
    /// assert_eq!(PlayerColor::Black, !PlayerColor::White);
    /// assert_eq!(PlayerColor::White, !PlayerColor::Black);
    /// ```
    fn not(self) -> Self::Output {
        match self
        {
            Self::Black => Self::White,
            Self::White => Self::Black,
        }
    }
}
