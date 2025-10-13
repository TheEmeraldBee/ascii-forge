use std::{io, time::Duration};

use ascii_forge::prelude::*;

fn main() -> io::Result<()> {
    let mut window_a = Window::init()?;
    window_a.set_cursor_visible(true);
    window_a.set_cursor_style(SetCursorStyle::BlinkingBar);

    loop {
        window_a.update(Duration::from_millis(500))?;

        render!(window_a,
            (0, 0) => [ "Controls:" ],
            (0, 1) => [ "hjkl: Move Cursor".green() ],
            (0, 2) => [ "H: Toggle Cursor Visibility".blue() ],
            (0, 3) => [ "b/B: Change Cursor Style".magenta() ],
            (0, 4) => [ "q: Quit".red() ],
        );

        if event!(window_a, Event::Key(k) => k.code == KeyCode::Char('q')) {
            break;
        } else if event!(window_a, Event::Key(k) => k.code == KeyCode::Char('h')) {
            window_a.move_cursor(-1, 0)
        } else if event!(window_a, Event::Key(k) => k.code == KeyCode::Char('l')) {
            window_a.move_cursor(1, 0)
        } else if event!(window_a, Event::Key(k) => k.code == KeyCode::Char('j')) {
            window_a.move_cursor(0, 1)
        } else if event!(window_a, Event::Key(k) => k.code == KeyCode::Char('k')) {
            window_a.move_cursor(0, -1)
        } else if event!(window_a, Event::Key(k) => k.code == KeyCode::Char('H')) {
            window_a.set_cursor_visible(!window_a.cursor_visible())
        } else if event!(window_a, Event::Key(k) => k.code == KeyCode::Char('b')) {
            window_a.set_cursor_style(SetCursorStyle::BlinkingBar);
        } else if event!(window_a, Event::Key(k) => k.code == KeyCode::Char('B')) {
            window_a.set_cursor_style(SetCursorStyle::BlinkingBlock);
        }
    }

    Ok(())
}
