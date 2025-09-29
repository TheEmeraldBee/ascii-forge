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
        }
    }
}

impl Render for Border {
    fn render(&self, loc: Vec2, buffer: &mut Buffer) -> Vec2 {
        if self.size.x < 3 || self.size.y < 3 {
            return loc;
        }
        for y in (loc.y + 1)..(loc.y + self.size.y.saturating_sub(1)) {
            buffer.set(vec2(loc.x, y), self.vertical);
            buffer.set(
                vec2(loc.x + self.size.x.saturating_sub(1), y),
                self.vertical,
            );
        }

        let _ = render!(buffer,
            loc => [self.top_left, self.horizontal.repeat(self.size.x.saturating_sub(2) as usize), self.top_right],
            vec2(loc.x, loc.y + self.size.y.saturating_sub(1)) => [self.bottom_left, self.horizontal.repeat(self.size.x.saturating_sub(2) as usize), self.bottom_right]
        );

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
