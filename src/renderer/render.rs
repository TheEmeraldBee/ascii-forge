use std::{fmt::Display, marker::PhantomData};

use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

use crate::prelude::*;

/// A macro to simplify rendering lots of items at once.
/// The Buffer can be anything that implements AsMut<Buffer>
/// This render will return the location of which the last element finished rendering.
/**
`Example`
```rust
# use ascii_forge::prelude::*;
# fn main() -> std::io::Result<()> {
// Create a buffer
let mut buffer = Buffer::new((32, 32));

// Render This works! and Another Element! To the window's buffer
render!(
    buffer,
        (16, 16) => [ "This works!" ],
        (0, 0) => [ "Another Element!" ]
);

# Ok(())
# }
```
*/
#[macro_export]
macro_rules! render {
    ($buffer:expr, $( $loc:expr => [$($render:expr),* $(,)?]),* $(,)?  ) => {{
        #[allow(unused_mut)]
        let mut loc;
        $(
            loc = Vec2::from($loc);
            $(loc = $render.render(loc, $buffer.as_mut()));*;
            let _ = loc;
        )*
        loc
    }};
}

/// The main system that will render an element at a location to the buffer.
/// Render's return type is the location the render ended at.
pub trait Render {
    fn render(&self, loc: Vec2, buffer: &mut Buffer) -> Vec2;
    fn size(&self) -> Vec2 {
        let mut buf = Buffer::new((u16::MAX, u16::MAX));
        render!(buf, vec2(0, 0) => [ self ]);
        buf.shrink();
        buf.size()
    }
}

/* --------------- Implementations --------------- */
impl Render for char {
    fn render(&self, mut loc: Vec2, buffer: &mut Buffer) -> Vec2 {
        buffer.set(loc, *self);
        loc.x += self.width().unwrap_or(1).saturating_sub(1) as u16;
        loc
    }
    fn size(&self) -> Vec2 {
        vec2(self.width().unwrap_or(1) as u16, 1)
    }
}

impl Render for &str {
    fn render(&self, loc: Vec2, buffer: &mut Buffer) -> Vec2 {
        render!(buffer, loc => [ StyledContent::new(ContentStyle::default(), self) ])
    }
    fn size(&self) -> Vec2 {
        StyledContent::new(ContentStyle::default(), self).size()
    }
}

impl<R: Render + 'static> From<R> for Box<dyn Render> {
    fn from(value: R) -> Self {
        Box::new(value)
    }
}

impl<R: Into<Box<dyn Render>> + Clone> Render for Vec<R> {
    fn render(&self, mut loc: Vec2, buffer: &mut Buffer) -> Vec2 {
        let items: Vec<Box<dyn Render>> = self.iter().map(|x| x.clone().into()).collect();
        for item in items {
            loc = render!(buffer, loc => [ item ]);
        }
        loc
    }
}

/// A Render type that doesn't get split. It purely renders the one item to the screen.
/// Useful for multi-character emojis.
pub struct CharString<D: Display, F: Into<StyledContent<D>> + Clone> {
    pub text: F,
    marker: PhantomData<D>,
}

impl<D: Display, F: Into<StyledContent<D>> + Clone> CharString<D, F> {
    pub fn new(text: F) -> Self {
        Self {
            text,
            marker: PhantomData {},
        }
    }
}

impl<D: Display, F: Into<StyledContent<D>> + Clone> Render for CharString<D, F> {
    fn render(&self, loc: Vec2, buffer: &mut Buffer) -> Vec2 {
        render!(buffer, loc => [ Cell::styled(self.text.clone().into()) ])
    }
}

impl Render for String {
    fn render(&self, loc: Vec2, buffer: &mut Buffer) -> Vec2 {
        render!(buffer, loc => [ self.as_str() ])
    }
}

impl<D: Display> Render for StyledContent<D> {
    fn render(&self, mut loc: Vec2, buffer: &mut Buffer) -> Vec2 {
        let base_x = loc.x;
        for line in format!("{}", self.content()).split('\n') {
            loc.x = base_x;
            for chr in line.chars().collect::<Vec<char>>() {
                buffer.set(loc, StyledContent::new(*self.style(), chr));
                loc.x += chr.width().unwrap_or(1) as u16;
            }
            loc.y += 1;
        }
        loc.y -= 1;
        loc
    }
    fn size(&self) -> Vec2 {
        let mut width = 0;
        let mut height = 0;
        for line in format!("{}", self.content()).split('\n') {
            width = line.chars().count().max(width);
            height += line.width() as u16;
        }
        vec2(width as u16, height)
    }
}
