use std::io;

use crossterm::{cursor, execute};

use crate::{
    cell::Cell,
    math::{vec2, Vec2},
    render::Render,
};

#[derive(Debug)]
pub struct Buffer {
    size: Vec2,
    cells: Vec<Cell>,
}

impl Buffer {
    pub fn new(size: Vec2) -> Self {
        Self {
            size,
            cells: vec![Cell::default(); size.x as usize * size.y as usize],
        }
    }

    pub fn size(&self) -> Vec2 {
        self.size
    }

    pub fn set<C: Into<Cell>>(&mut self, loc: Vec2, cell: C) {
        let idx = self.index_of(loc);

        self.cells[idx] = cell.into();
    }

    pub fn get(&self, loc: Vec2) -> &Cell {
        let idx = self.index_of(loc);
        &self.cells[idx]
    }

    pub fn get_mut(&mut self, loc: Vec2) -> &mut Cell {
        let idx = self.index_of(loc);
        &mut self.cells[idx]
    }

    fn index_of(&self, loc: Vec2) -> usize {
        let idx = loc.y as usize * self.size.x as usize + loc.x as usize;

        debug_assert!((idx as u16) < self.size.x * self.size.y);

        idx.min((self.size.x as usize * self.size.y as usize) - 1)
    }

    pub fn clear(&mut self) {
        *self = Self::new(self.size);
    }

    pub fn io_render(&self, io: &mut io::Stdout) -> io::Result<()> {
        for x in 0..self.size.x {
            for y in 0..self.size.y {
                let loc = vec2(x, y);

                execute!(io, cursor::MoveTo(loc.x, loc.y))?;

                print!("{}", self.get(loc));
            }
        }

        Ok(())
    }
}

impl Render for Buffer {
    fn render(&self, loc: Vec2, buffer: &mut Buffer) {
        for x in 0..self.size.x {
            if x + loc.x >= buffer.size.x {
                break;
            }

            for y in 0..self.size.y {
                if y + loc.y >= buffer.size.y {
                    break;
                }

                let dest = vec2(x + loc.x, y + loc.y);

                buffer.set(dest, self.get(vec2(x, y)).clone());
            }
        }
    }
}
