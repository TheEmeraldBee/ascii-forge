use std::{io, time::Duration};

use ascii_forge::prelude::*;

pub fn input<T, V>(validator: T) -> io::Result<V>
where
    T: Fn(&String) -> Option<V>,
{
    let mut window = Window::init_inline(2)?;

    let mut text = String::new();
    let mut status_text;

    loop {
        window.update(Duration::ZERO)?;

        if validator(&text).is_some() {
            status_text = "-- Valid --".green();
        } else {
            status_text = "-- Invalid --".red()
        }

        render!(window,
            vec2(0, 0) => [ status_text ],
            vec2(0, 1) => ["> ", text],
        );

        for event in window.events() {
            if let Event::Key(e) = event {
                match e.code {
                    KeyCode::Backspace => {
                        text.pop();
                    }
                    KeyCode::Enter => {
                        if let Some(t) = validator(&text) {
                            return Ok(t);
                        }
                    }
                    KeyCode::Char(c) => text.push(c),
                    _ => {}
                }
            }
        }

        if event!(window, Event::Key(e) => *e == KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL))
        {
            return Err(io::Error::new(
                io::ErrorKind::Interrupted,
                "Input validation canceled",
            ));
        }
    }
}

fn main() -> io::Result<()> {
    handle_panics();

    let num = input(|e| match e.parse::<i32>() {
        Ok(t) => Some(t),
        Err(_) => None,
    })
    .unwrap_or_default();

    println!("Input: {}", num);

    Ok(())
}
