use crate::prelude::*;

/// A basic border type.
/// Rendering this will put the next content inside of the function
pub struct Border {
    size: Vec2,
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
            size: vec2(width, height),
            horizontal: "─",
            vertical: "│",
            top_right: "┐",
            top_left: "┌",
            bottom_left: "└",
            bottom_right: "┘",
        }
    }

    // this is slightly faster than re-allocating the Border struct
    pub fn set_size(&mut self, size: impl Into<Vec2>) {
        self.size = size.into();
    }

}
impl Render for Border {
    fn render(&self, loc: Vec2, buffer: &mut Buffer) -> Vec2 {
        for y in (loc.y + 1)..(loc.y + self.size.y - 1) {
            buffer.set(vec2(loc.x, y), self.vertical);
            buffer.set(vec2(loc.x + self.size.x - 1, y), self.vertical);
        }

        let _ = render!(buffer,
            loc => [self.top_left, self.horizontal.repeat(self.size.x as usize - 2), self.top_right],
            vec2(loc.x, loc.y + self.size.y - 1) => [self.bottom_left, self.horizontal.repeat(self.size.x as usize - 2), self.bottom_right]
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
    fn check_size() {
        let border = Border::square(16, 16);
        let mut buf = Buffer::new((80, 80));
        render!(buf, (0, 0) => [ border ]);
        buf.shrink();
        assert_eq!(buf.size(), border.size())
    }
}
