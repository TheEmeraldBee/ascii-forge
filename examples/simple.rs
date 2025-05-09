use ascii_forge::prelude::*;
use std::{io, time::Duration};

fn main() -> io::Result<()> {
    // Will init the window for you, handling all required procedures.
    let mut window = Window::init()?;

    // Ask the system to handle panics for us.
    handle_panics();

    loop {
        // Ask the window to draw, handle events, and fix sizing issues.
        // Duration is the time for which to poll events before re-rendering.
        window.update(Duration::from_millis(200))?;

        // Render elements to the window
        render!(
            window,
            vec2(0, 0) => [ "Hello World!" ],
            vec2(0, 1) => [ "Press `Enter` to exit!".red() ],
            vec2(0, 2) => [
                "Render ".red(),
                "Multiple ".yellow(),
                "Elements ",
                "In one go!".to_string()
            ],
        );

        // Check if the Enter Key was pressed, and exit the app if it was.
        if event!(window, Event::Key(e) => e.code == KeyCode::Enter) {
            break;
        }
    }

    // Restore the window, enabling the window to function normally again
    // If nothing will be run after this, once the window is dropped, this will be run implicitly.
    window.restore()
}
