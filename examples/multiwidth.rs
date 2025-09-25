use ascii_forge::prelude::*;
use std::{io, time::Duration};

fn main() -> io::Result<()> {
    let mut window = Window::init()?;
    handle_panics();

    let buf = Buffer::sized_element("Normal: Hello World!\nWide: 👩‍👩‍👧‍👦 and 🚀\nMixed: a👩‍👩‍👧‍👦b🚀c");

    let buf_size = buf.size();

    let mut pos = (0i16, 0i16);
    let mut vel = (1i16, 1i16);

    loop {
        window.update(Duration::from_millis(60))?;
        let win_size = window.size();
        let max_x = win_size.x as i16;
        let max_y = win_size.y as i16;

        pos.0 += vel.0;
        pos.1 += vel.1;

        if pos.0 < 0 {
            pos.0 = 0;
            vel.0 = -vel.0;
        } else if pos.0 + buf_size.x as i16 >= max_x {
            pos.0 = max_x - buf_size.x as i16;
            vel.0 = -vel.0;
        }

        if pos.1 < 0 {
            pos.1 = 0;
            vel.1 = -vel.1;
        } else if pos.1 + buf_size.y as i16 >= max_y {
            pos.1 = max_y - buf_size.y as i16;
            vel.1 = -vel.1;
        }

        render!(
            window,
            vec2(pos.0 as u16, pos.1 as u16) => [ buf ],
            vec2(0, max_y.saturating_sub(2) as u16) => [ "Press `Enter` to exit!".red() ],
        );

        if event!(window, Event::Key(e) => e.code == KeyCode::Enter) {
            break;
        }
    }

    window.restore()
}
