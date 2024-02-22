![](https://github.com/TheEmeraldBee/ascii-forge/blob/master/logo.png?raw=true)

# Ascii-Forge
A dead simple Terminal UI Engine built off of crossterm that improves terminal UI/Games without adding too much fluff.

# Why?
Although other terminal UI Engines already exist, like [Ratatui](https://github.com/ratatui-org/ratatui), I felt there was a lot of fluff that wasn't needed for a small application.

As well, there aren't many bare-bones terminal UI engines with as much flexability as would be needed to make a fun game. In order to acomplish this, all elements of the engine are available, at all times.

# What?
Using crossterm, I built a rendering engine for ascii that allows you to rapidly build apps using a terminal.

# Examples
Most of the examples will be found in the [examples](https://github.com/TheEmeraldBee/ascii-forge/tree/master/examples) directory

Simplest Example Included for convenience
```rust
use std::{io, time::Duration};

use ascii_forge::prelude::*;

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
        render!(window,
            vec2(0, 0) => [ "Hello World!" ],
            vec2(0, 1) => [ "Press `Enter` to exit!".red() ],
        );

        // Check if the Enter Key was pressed, and exit the app if it was.
        if window.code(KeyCode::Enter) {
            break;
        }
    }

    // Crucial to call before exiting the program, as otherwise you will not leave the alternate screen.
    window.restore()
}
```

# Documentation
- [docs.rs](https://docs.rs/ascii-forge/latest/ascii_forge/)
- Wiki Coming Soon!
