use std::{error::Error, time::Duration};

use ascii_forge::prelude::*;
use crossterm::execute;

fn main() -> Result<(), Box<dyn Error>> {
    let mut event = Event::FocusGained;
    let mut window = Window::init()?;

    if !window.supports().keyboard() {
        window.restore()?;
        eprintln!("This game does not support this terminal.\nIf Curious, look up terminals that support the kitty keyboard protocol");
        return Err("Terminal Unsupported".into());
    }

    execute!(
        window.io(),
        PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::all())
    )?;

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

    execute!(window.io(), PopKeyboardEnhancementFlags)?;

    Ok(())
}
