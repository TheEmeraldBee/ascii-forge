use std::{error::Error, time::Duration};

use ascii_forge::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let mut event = Event::FocusGained;
    let mut window = Window::init()?;

    // Require the window to support kitty keyboard protocol
    window.supports().keyboard()?;

    loop {
        window.update(Duration::ZERO)?;

        if let Some(new_event) = window.event() {
            event = new_event.clone();
        }

        render!(
            window, [
                vec2(0, 20) => "To Quit, Press Ctrl + C".red(),
                vec2(0, 0) => format!("{:#?}", event).replace('\t', "   "),
            ]
        );

        if window.key(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL)) {
            break;
        }
    }

    Ok(())
}
