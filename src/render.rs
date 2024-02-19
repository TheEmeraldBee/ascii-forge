use std::fmt::Display;

use crossterm::style::StyledContent;

use crate::{buffer::Buffer, math::Vec2};

#[macro_export]
macro_rules! render {
    ($buffer:expr, [$($loc:expr => $render:expr),* $(,)?]) => {
        $(
            $render.render($loc, $buffer);
        )*
    };
}

pub trait Render {
    fn render(&self, loc: Vec2, buffer: &mut Buffer);
}

/* --------------- Implementations --------------- */
impl Render for char {
    fn render(&self, loc: Vec2, buffer: &mut Buffer) {
        buffer.set(loc, *self)
    }
}

impl Render for &str {
    fn render(&self, mut loc: Vec2, buffer: &mut Buffer) {
        let base_x = loc.x;
        for line in self.split('\n') {
            for chr in line.trim().chars().collect::<Vec<char>>() {
                buffer.set(loc, chr);
                loc.x += 1;
            }
            loc.y += 1;
            loc.x = base_x;
        }
    }
}

impl Render for String {
    fn render(&self, loc: Vec2, buffer: &mut Buffer) {
        render!(buffer, [loc => self.as_str()]);
    }
}

impl Render for Vec<String> {
    fn render(&self, loc: Vec2, buffer: &mut Buffer) {
        render!(buffer, [loc => self.join("\n")]);
    }
}

impl Render for Vec<&str> {
    fn render(&self, loc: Vec2, buffer: &mut Buffer) {
        render!(buffer, [loc => self.join("\n")]);
    }
}

impl<D: Display> Render for StyledContent<D> {
    fn render(&self, mut loc: Vec2, buffer: &mut Buffer) {
        let base_x = loc.x;
        for line in format!("{}", self.content()).split('\n') {
            for char in line.trim().chars().collect::<Vec<char>>() {
                buffer.set(loc, StyledContent::new(*self.style(), char));
                loc.x += 1;
            }
            loc.y += 1;
            loc.x = base_x;
        }
    }
}
