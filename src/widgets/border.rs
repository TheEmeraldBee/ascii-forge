use crate::prelude::*;

/// A basic border type.
/// Rendering this will put the next content inside of the function
pub struct Border {
    width: u16,
    height: u16,
    horizontal: &'static str,
    vertical: &'static str,
    top_left: &'static str,
    top_right: &'static str,
    bottom_left: &'static str,
    bottom_right: &'static str,
}

impl Border {
    pub const fn square(width: u16, height: u16) -> Border {
        Border {
            width,
            height,
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
        for y in (loc.y + 1)..(loc.y + self.height - 1) {
            buffer.set(vec2(loc.x, y), self.vertical);
            buffer.set(vec2(loc.x + self.width - 1, y), self.vertical);
        }

        let _ = render!(buffer,
            loc => [self.top_left, self.horizontal.repeat(self.width as usize - 2), self.top_right],
            vec2(loc.x, loc.y + self.height - 1) => [self.bottom_left, self.horizontal.repeat(self.width as usize - 2), self.bottom_right]
        );

        vec2(loc.x + 1, loc.y + 1)
    }
    fn size(&self) -> Vec2 {
        vec2(self.width, self.height)
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
    fn check_size() {
        let border = Border::square(16, 16);
        let mut buf = Buffer::new((80, 80));
        render!(buf, (0, 0) => [ border ]);
        buf.shrink();
        assert_eq!(buf.size(), border.size())
    }
}
