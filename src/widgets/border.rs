use std::ops::{Deref, DerefMut};

use crate::prelude::*;

/// A basic border type.
/// Rendering this will put the next content inside of the function
/// Borders will skip rendering if their size is under a 3x3
pub struct Border {
    pub size: Vec2,
    pub horizontal: &'static str,
    pub vertical: &'static str,
    pub top_left: &'static str,
    pub top_right: &'static str,
    pub bottom_left: &'static str,
    pub bottom_right: &'static str,

    pub title: Option<Buffer>,

    pub style: ContentStyle,
}

impl Deref for Border {
    type Target = ContentStyle;
    fn deref(&self) -> &Self::Target {
        &self.style
    }
}

impl DerefMut for Border {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.style
    }
}

impl Border {
    pub const fn square(width: u16, height: u16) -> Border {
        Border {
            size: vec2(width, height),
            horizontal: "─",
            vertical: "│",
            top_right: "┐",
            top_left: "┌",
            bottom_left: "└",
            bottom_right: "┘",

            title: None,

            style: ContentStyle {
                foreground_color: None,
                background_color: None,
                underline_color: None,
                attributes: Attributes::none(),
            },
        }
    }

    pub const fn rounded(width: u16, height: u16) -> Border {
        Border {
            size: vec2(width, height),
            horizontal: "─",
            vertical: "│",
            top_right: "╮",
            top_left: "╭",
            bottom_left: "╰",
            bottom_right: "╯",

            title: None,

            style: ContentStyle {
                foreground_color: None,
                background_color: None,
                underline_color: None,
                attributes: Attributes::none(),
            },
        }
    }

    pub const fn thick(width: u16, height: u16) -> Border {
        Border {
            size: vec2(width, height),
            horizontal: "━",
            vertical: "┃",
            top_right: "┓",
            top_left: "┏",
            bottom_left: "┗",
            bottom_right: "┛",

            title: None,

            style: ContentStyle {
                foreground_color: None,
                background_color: None,
                underline_color: None,
                attributes: Attributes::none(),
            },
        }
    }

    pub const fn double(width: u16, height: u16) -> Border {
        Border {
            size: vec2(width, height),
            horizontal: "═",
            vertical: "║",
            top_right: "╗",
            top_left: "╔",
            bottom_left: "╚",
            bottom_right: "╝",

            title: None,

            style: ContentStyle {
                foreground_color: None,
                background_color: None,
                underline_color: None,
                attributes: Attributes::none(),
            },
        }
    }

    pub fn with_title(mut self, title: impl Render) -> Border {
        let title_buf = Buffer::sized_element(title);
        self.title = Some(title_buf);

        self
    }
}

impl Render for Border {
    fn render(&self, loc: Vec2, buffer: &mut Buffer) -> Vec2 {
        if self.size.x < 3 || self.size.y < 3 {
            return loc;
        }

        // Fill the interior with spaces
        for y in (loc.y + 1)..(loc.y + self.size.y.saturating_sub(1)) {
            for x in (loc.x + 1)..(loc.x + self.size.x.saturating_sub(1)) {
                buffer.set(vec2(x, y), " ");
            }
        }

        // Render vertical sides with style
        for y in (loc.y + 1)..(loc.y + self.size.y.saturating_sub(1)) {
            buffer.set(
                vec2(loc.x, y),
                StyledContent::new(self.style, self.vertical),
            );
            buffer.set(
                vec2(loc.x + self.size.x.saturating_sub(1), y),
                StyledContent::new(self.style, self.vertical),
            );
        }

        // Render top and bottom borders with style
        let horizontal_repeat = self
            .horizontal
            .repeat(self.size.x.saturating_sub(2) as usize);
        render!(buffer,
            loc => [
                StyledContent::new(self.style, self.top_left),
                StyledContent::new(self.style, horizontal_repeat.as_str()),
                StyledContent::new(self.style, self.top_right)
            ],
            vec2(loc.x, loc.y + self.size.y.saturating_sub(1)) => [
                StyledContent::new(self.style, self.bottom_left),
                StyledContent::new(self.style, self.horizontal.repeat(self.size.x.saturating_sub(2) as usize).as_str()),
                StyledContent::new(self.style, self.bottom_right)
            ]
        );

        // Render title with clipping to fit within the border width
        if let Some(title) = &self.title {
            let max_title_width = self.size.x.saturating_sub(2); // Account for corners
            title.render_clipped(loc + vec2(1, 0), vec2(max_title_width, 1), buffer);
        }

        vec2(loc.x + 1, loc.y + 1)
    }

    fn size(&self) -> Vec2 {
        self.size
    }
}

#[cfg(test)]
mod test {
    use crate::{
        math::Vec2,
        render,
        widgets::border::Border,
        window::{Buffer, Render},
    };

    #[test]
    fn render_small() {
        let border = Border::square(0, 0);
        // Ensure no panics
        let _ = Buffer::sized_element(border);
    }

    #[test]
    fn check_size() {
        let border = Border::square(16, 16);
        let mut buf = Buffer::new((80, 80));
        render!(buf, (0, 0) => [ border ]);
        buf.shrink();
        assert_eq!(buf.size(), border.size())
    }
}
