use ascii_forge::prelude::*;
use crossterm::event::*;
use crossterm::style::*;
use std::{
    io,
    time::{Duration, SystemTime},
};

fn progress_bar() -> io::Result<()> {
    let mut window = Window::init_inline(2)?;

    let timer = SystemTime::now();
    let duration = Duration::from_secs(3);

    // The Inline Render Loop
    loop {
        // Render's the Window and captures events
        window.update(Duration::ZERO)?;

        let amount_done = SystemTime::now().duration_since(timer).unwrap();

        let percent = amount_done.as_secs_f64() / duration.as_secs_f64();

        if percent >= 1.0 {
            break;
        }

        let x = (window.size().x as f64 * percent).round() as u16;

        // Create the progress bar text
        let text_green = "|".repeat(x as usize).green();
        let text_red = "|".repeat((window.size().x - x) as usize).red();

        // Render the Progress Bar
        render!(window,
            vec2(0, 1) => [ text_green ],
            vec2(x, 1) => [ text_red ],
            vec2(0, 0) => [ "Progress" ],
        );

        // End the loop if key is pressed early
        if event!(window, Event::Key(e) => *e == KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL))
        {
            break;
        }
    }

    window.restore()
}

fn main() -> io::Result<()> {
    // Start by asking the terminal to handle if a panic happens.
    handle_panics();

    // Render The Progress bar.
    progress_bar()?;

    println!("Progress bar complete!");

    Ok(())
}
