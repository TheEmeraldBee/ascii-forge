use std::{error::Error, time::Duration};

use ascii_forge::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let mut event = Event::FocusGained;
    let mut window = Window::<KittyInput>::init()?;

    loop {
        window.update(Duration::ZERO)?;

        if let Some(new_event) = window.events().last() {
            event = new_event.clone();
        }

        render!(
            window,
                vec2(0, 20) => [ "To Quit, Press Ctrl + C".red() ],
                vec2(0, 0) => [ format!("{:#?}", event).replace('\t', "   ") ],
        );

        if window
            .input()
            .pressed_mod(KeyCode::Char('c'), KeyModifiers::CONTROL)
        {
            break;
        }
    }

    Ok(())
}
