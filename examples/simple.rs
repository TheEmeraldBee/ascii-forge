use std::error::Error;

use ascii_forge::prelude::crossterm::{cursor, event::KeyCode, style::Stylize};
use ascii_forge::prelude::*;

// Create the main scene.
pub struct MainScene;

impl Scene for MainScene {
    fn run(&mut self, window: &mut Window) -> Result<SceneResult, Box<dyn Error>> {
        // Set up a scene
        let mut ui_buffer = Buffer::new((6, 3));

        let chars = ['┌', '─', '┐', '│', ' ', '│', '└', '─', '┘'];
        let nine_slice = ui::NineSlice::new(chars, (6, 3));

        // Render Some elements to the buffer.
        render!(ui_buffer, [
            vec2(0, 0) => nine_slice,
            vec2(1, 1) => "QUIT".green()
        ]);

        // Loop the current scene.
        loop {
            window.update()?;

            // Renders some elements to the window.
            render!(window, [
                vec2(0, 0) => format!("{:?}", cursor::position()?),
                vec2(window.size().x / 2, window.size().y / 4) => ui_buffer,
            ]);

            // If the `q` key was pressed, quit the application by returning no scene.
            if window.code(KeyCode::Char('q')) {
                return Ok(None);
            }

            // If the buffer was clicked on, quit.
            if window.mouse(MouseEventKind::Down(MouseButton::Left))?
                && window.hover(
                    vec2(window.size().x / 2, window.size().y / 4),
                    ui_buffer.size(),
                )?
            {
                return Ok(None);
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Run the application, with the given starting scene.
    app(MainScene)
}
