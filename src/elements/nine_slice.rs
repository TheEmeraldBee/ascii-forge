use crate::prelude::*;

#[derive(Clone, Debug)]
pub struct NineSlice {
    cells: [Cell; 9],
    width: u16,
    height: u16,
}

impl NineSlice {
    pub fn new<C: Into<Cell>>(cells: [C; 9], size: impl Into<Vec2>) -> Self {
        let size = size.into();
        Self {
            cells: cells
                .into_iter()
                .map(|x| x.into())
                .collect::<Vec<Cell>>()
                .try_into()
                .expect("Cells length should be 9"),
            width: size.x - 1,
            height: size.y - 1,
        }
    }
}

impl Render for NineSlice {
    fn render(&self, loc: crate::prelude::Vec2, buffer: &mut crate::prelude::Buffer) {
        let top = loc.y;
        let bottom = loc.y + self.height;

        // Render Top and Bottom
        for x in (loc.x + 1)..(loc.x + self.width) {
            render!(
                buffer, [
                    vec2(x, top) => self.cells[1],
                    vec2(x, bottom) => self.cells[7]
                ]
            );

            for y in (loc.y + 1)..(loc.y + self.height) {
                render!(
                    buffer, [
                        vec2(x, y) => self.cells[4]
                    ]
                );
            }
        }

        let left = loc.x;
        let right = loc.x + self.width;

        for y in (loc.y + 1)..(loc.y + self.height) {
            render!(
                buffer, [
                    vec2(left, y) => self.cells[3],
                    vec2(right, y) => self.cells[5]
                ]
            );
        }

        // Render Corners
        render!(
            buffer, [
                loc => self.cells[0],
                vec2(loc.x + self.width, loc.y) => self.cells[2],
                vec2(loc.x, loc.y + self.height) => self.cells[6],
                vec2(loc.x + self.width, loc.y + self.height) => self.cells[8],
            ]
        );
    }
}
