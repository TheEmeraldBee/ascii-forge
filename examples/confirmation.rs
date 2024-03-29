use ascii_forge::prelude::*;
use std::{io, time::Duration};

pub fn confirmation() -> io::Result<bool> {
    let mut window = Window::init_inline(1)?;
    window.keyboard()?;

    loop {
        render!(window, vec2(0, 0) => [ "Are You Sure? (`y` / `n`)" ]);

        if event!(window, Event::Key(e) => e.code == KeyCode::Char('Y') || e.code == KeyCode::Char('y'))
        {
            return Ok(true);
        }

        if event!(window, Event::Key(e) => e.code == KeyCode::Char('n') || e.code == KeyCode::Char('N'))
        {
            return Ok(false);
        }

        // Update the window
        window.update(Duration::from_millis(1000))?;
    }
}

pub fn standard_confirmation() -> io::Result<bool> {
    println!("Are you Sure? (`Y` / `N`)");
    loop {
        let mut input = String::new();

        io::stdin().read_line(&mut input)?;

        if input.trim().to_lowercase() == *"y" {
            return Ok(true);
        }
        if input.trim().to_lowercase() == *"n" {
            return Ok(false);
        }
        println!(
            "Invalid Input {}, please input either `Y` or `N`",
            input.trim()
        );
    }
}

fn main() -> io::Result<()> {
    println!("State: {}", standard_confirmation()?);

    println!("State: {}", confirmation()?);
    Ok(())
}
