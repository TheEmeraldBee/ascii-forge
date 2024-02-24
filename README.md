![](https://github.com/TheEmeraldBee/ascii-forge/blob/master/logo.png?raw=true)

# Ascii-Forge
An oppinionated terminal canvas rendering engine built off of crossterm with the goal of improving terminal UI/Games without adding any un-needed elements.

# Why?
Although other terminal UI Engines already exist, like [Ratatui](https://github.com/ratatui-org/ratatui), I felt there was a lot of extra elements that wasn't needed for a small application or game.

As well, there aren't many bare-bones terminal canvas engines with as much flexability as would be needed to make a fun game. In order to acomplish this, all elements of the engine are available, at all times.

# But What is Different?
As said before, Ascii-Forge is oppinionated, you don't have a choice of the backend, crossterm is what you get, but it is the best, and one of the only fully cross-platform terminal engines.

To list off some big differences:
- Keeping it as small as possible while still making things easy.
- Absolutely everything used to make the engine available is available to you.
    - This means that if the update method doesn't work as expected, you can make your own using the other methods.
    - Want to access the stdout that the window is using, use the `io()` method!
- Most of the larger engines make their own layout system, this doesn't. You use columns and rows, no extra abstraction on top of this.

# Examples
Most of the examples will be found in the [examples](https://github.com/TheEmeraldBee/ascii-forge/tree/master/examples) directory

Simplest Example Included Here.
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
            vec2(0, 2) => [
                "Render ".red(),
                "Multiple ".yellow(),
                "Elements ",
                "In one go!".to_string()
            ]
        );

        // Check if the Enter Key was pressed, and exit the app if it was.
        if window.input().code(KeyCode::Enter) {
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
