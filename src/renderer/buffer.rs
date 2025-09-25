use crate::prelude::*;

/**
A screen buffer that can be rendered to, has a size

This is the backbone of ascii-forge

`Example`
```rust, no_run
use ascii_forge::prelude::*;

// A 30x30 buffer window
let mut buffer = Buffer::new(30, 30);

// Render Hello World to the top left of the buffer
render!(
    buffer, {
        (0, 0) => "Hello World!"
    }
);
```

*/
#[derive(Debug)]
pub struct Buffer {
    size: Vec2,
    cells: Vec<Cell>,
}

impl AsMut<Buffer> for Buffer {
    fn as_mut(&mut self) -> &mut Buffer {
        self
    }
}

impl Buffer {
    /// Creates a new buffer of empty cells with the given size.
    pub fn new(size: impl Into<Vec2>) -> Self {
        let size = size.into();
        Self {
            size,
            cells: vec![Cell::default(); size.x as usize * size.y as usize],
        }
    }

    /// Returns the current size of the buffer.
    pub fn size(&self) -> Vec2 {
        self.size
    }

    /// Sets a cell at the given location to the given cell
    pub fn set<C: Into<Cell>>(&mut self, loc: impl Into<Vec2>, cell: C) {
        let loc = loc.into();
        let idx = self.index_of(loc);

        // Ignore if cell is out of bounds
        let Some(idx) = idx else {
            return;
        };

        let cell = cell.into();

        // Overwrite the next cell if the character is wide
        if cell.width() > 1 {
            self.set(loc + vec2(1, 0), Cell::default());
        }

        self.cells[idx] = cell;
    }

    /// Sets all cells at the given location to the given cell
    pub fn fill<C: Into<Cell>>(&mut self, cell: C) {
        let cell = cell.into();
        for i in 0..self.cells.len() {
            self.cells[i] = cell.clone()
        }
    }

    /// Returns a reverence to the cell at the given location.
    pub fn get(&self, loc: impl Into<Vec2>) -> Option<&Cell> {
        let idx = self.index_of(loc)?;
        self.cells.get(idx)
    }

    /// Returns a mutable reference to the cell at the given location.
    pub fn get_mut(&mut self, loc: impl Into<Vec2>) -> Option<&mut Cell> {
        let idx = self.index_of(loc)?;
        self.cells.get_mut(idx)
    }

    fn index_of(&self, loc: impl Into<Vec2>) -> Option<usize> {
        let loc = loc.into();
        let idx = loc.y as usize * self.size.x as usize + loc.x as usize;

        if (idx as u16) >= self.size.x * self.size.y {
            return None;
        }

        Some(idx.min((self.size.x as usize * self.size.y as usize) - 1))
    }

    /// Clears the buffer
    pub fn clear(&mut self) {
        *self = Self::new(self.size);
    }

    /// Returns the cells and locations that are different between the two buffers
    pub fn diff<'a>(&self, other: &'a Buffer) -> Vec<(Vec2, &'a Cell)> {
        assert!(self.size == other.size);

        let mut res = vec![];
        let mut skip = 0;

        for x in 0..self.size.x {
            for y in 0..self.size.y {
                if skip > 0 {
                    skip -= 1;
                    continue;
                }

                let old = self.get((x, y));
                let new = other.get((x, y));

                if old != new {
                    if let Some(new) = new {
                        skip = new.width().saturating_sub(1) as usize;
                        res.push((vec2(x, y), new))
                    }
                }
            }
        }

        res
    }

    /// Shrinks the buffer to the given size by dropping any cells that are only whitespace
    pub fn shrink(&mut self) {
        let mut max_whitespace_x = 0;
        let mut max_whitespace_y = 0;
        for x in (0..self.size.x).rev() {
            for y in (0..self.size.y).rev() {
                if !self
                    .get((x, y))
                    .expect("Cell should be in bounds")
                    .is_empty()
                {
                    max_whitespace_x = x.max(max_whitespace_x);
                    max_whitespace_y = y.max(max_whitespace_y);
                }
            }
        }

        self.resize(vec2(max_whitespace_x + 1, max_whitespace_y + 1));
    }

    /// Resizes the buffer while retaining elements that have already been rendered
    pub fn resize(&mut self, new_size: impl Into<Vec2>) {
        let new_size = new_size.into();
        if self.size == new_size {
            return;
        }

        let mut new_elements = vec![];

        for y in 0..new_size.y {
            for x in 0..new_size.x {
                new_elements.push(self.get((x, y)).expect("Cell should be in bounds").clone());
            }
        }

        self.size = new_size;
        self.cells = new_elements;
    }

    /// Creates a Buffer from the given element with the minimum size it could have for that element.
    /// Useful for if you want to store any set of render elements in a custom element.
    pub fn sized_element<R: Render>(item: R) -> Self {
        let mut buff = Buffer::new((100, 100));
        render!(buff, vec2(0, 0) => [ item ]);
        buff.shrink();
        buff
    }
}

impl Render for Buffer {
    fn render(&self, loc: Vec2, buffer: &mut Buffer) -> Vec2 {
        for x in 0..self.size.x {
            if x + loc.x >= buffer.size.x {
                break;
            }

            for y in 0..self.size.y {
                if y + loc.y >= buffer.size.y {
                    break;
                }

                let dest = vec2(x + loc.x, y + loc.y);

                buffer.set(
                    dest,
                    self.get(vec2(x, y))
                        .expect("Cell should be in bounds")
                        .clone(),
                );
            }
        }
        vec2(loc.x + buffer.size().x, loc.y + buffer.size().y)
    }
}
