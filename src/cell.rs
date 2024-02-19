use std::fmt::Display;

use compact_str::{CompactString, ToCompactString};
use crossterm::style::{ContentStyle, StyledContent};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Cell {
    text: CompactString,
    style: ContentStyle,
}

impl Default for Cell {
    fn default() -> Self {
        Self::string(" ")
    }
}

impl Cell {
    pub fn new<S: Into<ContentStyle>>(text: impl Into<CompactString>, style: S) -> Self {
        Self {
            text: text.into(),
            style: style.into(),
        }
    }

    pub fn string(string: impl AsRef<str>) -> Self {
        Self {
            text: CompactString::new(string),
            style: ContentStyle::default(),
        }
    }

    pub fn chr(chr: char) -> Self {
        Self {
            text: chr.to_compact_string(),
            style: ContentStyle::default(),
        }
    }

    pub fn style<D: Display>(content: StyledContent<D>) -> Self {
        Self {
            text: CompactString::new(format!("{}", content.content())),
            style: *content.style(),
        }
    }
}

macro_rules! str_impl {
    ($($ty:ty)*) => {
        $(
            impl From<$ty> for Cell {
                fn from(value: $ty) -> Self {
                    Self::string(value)
                }
            }
        )*
    };
}

str_impl! {&str String}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        Self::chr(value)
    }
}

impl<D: Display> From<StyledContent<D>> for Cell {
    fn from(value: StyledContent<D>) -> Self {
        Self::style(value)
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", StyledContent::new(self.style, &self.text))
    }
}
