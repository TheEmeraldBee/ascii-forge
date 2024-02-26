use std::{fmt::Display, io, time::Duration};

use ascii_forge::prelude::*;
use regex::Regex;

pub fn input<T, V>(validator: T) -> io::Result<V>
where
    T: Fn(&str) -> Option<V>,
{
    let mut window = Window::init_inline(2)?;

    let mut text = String::new();
    let mut status_text;

    loop {
        window.update(Duration::ZERO)?;

        if validator(&text).is_some() {
            status_text = "-- Valid --".green();
        } else {
            status_text = "-- Invalid --".red();
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

#[derive(Debug)]
pub struct Email {
    pub prefix: String,
    pub suffix: String,
}

impl Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}@{}", self.prefix, self.suffix)
    }
}

fn email(text: &str) -> Option<Email> {
    let mut email = Email {
        prefix: "".to_string(),
        suffix: "".to_string(),
    };

    let regex = match Regex::new(r"^(?<prefix>[\w\-\.]+)@(?<suffix>[\w-]+\.+[\w-]{2,4})$")
        .expect("Regex should be fine")
        .captures(text)
    {
        Some(s) => s,
        None => return None,
    };

    if let Some(item) = regex.name("prefix") {
        email.prefix = item.as_str().to_string();
    } else {
        return None;
    }

    if let Some(item) = regex.name("suffix") {
        email.suffix = item.as_str().to_string();
    } else {
        return None;
    }

    Some(email)
}

fn main() -> io::Result<()> {
    handle_panics();

    println!("Input your age!");
    let num = match input(|e| match e.parse::<i32>() {
        Ok(t) => Some(t),
        Err(_) => None,
    }) {
        Ok(t) => t,
        Err(_) => return Ok(()),
    };

    println!("Input your email!");
    let email = match input(email) {
        Ok(t) => t,
        Err(_) => return Ok(()),
    };

    println!("{num}, {email}");

    Ok(())
}
