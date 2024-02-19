use std::error::Error;

use ascii_forge::prelude::*;
use crossterm::{
    event::{KeyCode, KeyEvent, KeyModifiers},
    style::Stylize,
};

pub struct MainScene;

impl Scene for MainScene {
    fn run(&mut self, window: &mut Window) -> Result<SceneResult, Box<dyn Error>> {
        loop {
            window.update()?;

            let mut my_buffer = Buffer::new(vec2(19, 32));
            render!(&mut my_buffer, [
                vec2(0, 0) => "custom screen\0 works!".on_blue().red()
            ]);

            render!(window.buffer(), [
                vec2(window.size().x / 2, window.size().y / 2) => my_buffer,
                vec2(window.size().x / 3, 0) => my_buffer,
                vec2(0, 0) => format!("{:?}", window.event()).underline_green(),
            ]);

            if window.key_code(KeyCode::Char('q')) {
                return Ok(None);
            }

            if window.key(KeyEvent::new(KeyCode::Char('p'), KeyModifiers::SHIFT)) {
                panic!("Panic Test");
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    app(MainScene)
}
