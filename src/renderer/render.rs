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

/// The main trait that allows for rendering an element at a location to the buffer.
/// Render's return type is the location the render ended at.
pub trait Render {
    /// Render the object to the buffer at the given location.
    fn render(&self, loc: Vec2, buffer: &mut Buffer) -> Vec2;

    /// Returns the resulting size of the element
    fn size(&self) -> Vec2 {
        let mut buf = Buffer::new((u16::MAX, u16::MAX));
        render!(buf, vec2(0, 0) => [ self ]);
        buf.shrink();
        buf.size()
    }

    /// Render's the element into a clipped view, allowing for clipping easily
    fn render_clipped(&self, loc: Vec2, clip_size: Vec2, buffer: &mut Buffer) -> Vec2 {
        let mut buff = Buffer::new((100, 100));
        render!(buff, vec2(0, 0) => [ self ]);
        buff.shrink();

        buff.render_clipped(loc, clip_size, buffer)
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

    fn render_clipped(&self, loc: Vec2, clip_size: Vec2, buffer: &mut Buffer) -> Vec2 {
        let char_width = self.width().unwrap_or(1) as u16;

        // Only render if there's enough space for the character
        if clip_size.x >= char_width && clip_size.y >= 1 {
            buffer.set(loc, *self);
            vec2(loc.x + char_width, loc.y)
        } else {
            loc
        }
    }
}

impl Render for &str {
    fn render(&self, loc: Vec2, buffer: &mut Buffer) -> Vec2 {
        render!(buffer, loc => [ StyledContent::new(ContentStyle::default(), self) ])
    }

    fn size(&self) -> Vec2 {
        StyledContent::new(ContentStyle::default(), self).size()
    }

    fn render_clipped(&self, loc: Vec2, clip_size: Vec2, buffer: &mut Buffer) -> Vec2 {
        StyledContent::new(ContentStyle::default(), self).render_clipped(loc, clip_size, buffer)
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

    fn render_clipped(&self, mut loc: Vec2, clip_size: Vec2, buffer: &mut Buffer) -> Vec2 {
        let start_loc = loc;
        let items: Vec<Box<dyn Render>> = self.iter().map(|x| x.clone().into()).collect();

        for item in items {
            // Calculate remaining clip space
            let used_x = loc.x.saturating_sub(start_loc.x);
            let used_y = loc.y.saturating_sub(start_loc.y);

            if used_y >= clip_size.y {
                break;
            }

            let remaining_clip = vec2(
                clip_size.x.saturating_sub(used_x),
                clip_size.y.saturating_sub(used_y),
            );

            if remaining_clip.x == 0 || remaining_clip.y == 0 {
                break;
            }

            loc = item.render_clipped(loc, remaining_clip, buffer);
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

    fn render_clipped(&self, loc: Vec2, clip_size: Vec2, buffer: &mut Buffer) -> Vec2 {
        let cell = Cell::styled(self.text.clone().into());
        let cell_width = cell.width();

        // Only render if there's enough space for the entire cell
        if clip_size.x >= cell_width && clip_size.y >= 1 {
            buffer.set(loc, cell);
            vec2(loc.x + cell_width, loc.y)
        } else {
            loc
        }
    }
}

impl Render for String {
    fn render(&self, loc: Vec2, buffer: &mut Buffer) -> Vec2 {
        render!(buffer, loc => [ self.as_str() ])
    }

    fn render_clipped(&self, loc: Vec2, clip_size: Vec2, buffer: &mut Buffer) -> Vec2 {
        self.as_str().render_clipped(loc, clip_size, buffer)
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

    fn render_clipped(&self, mut loc: Vec2, clip_size: Vec2, buffer: &mut Buffer) -> Vec2 {
        let base_x = loc.x;
        let start_y = loc.y;
        let mut lines_rendered = 0;

        for line in format!("{}", self.content()).split('\n') {
            if lines_rendered >= clip_size.y {
                break;
            }

            loc.x = base_x;
            let mut chars_rendered = 0;

            for chr in line.chars().collect::<Vec<char>>() {
                let chr_width = chr.width().unwrap_or(1) as u16;

                if chars_rendered + chr_width > clip_size.x {
                    break;
                }

                buffer.set(loc, StyledContent::new(*self.style(), chr));
                loc.x += chr_width;
                chars_rendered += chr_width;
            }

            loc.y += 1;
            lines_rendered += 1;
        }

        vec2(
            base_x + lines_rendered.min(clip_size.x),
            start_y + lines_rendered.min(clip_size.y),
        )
    }
}
