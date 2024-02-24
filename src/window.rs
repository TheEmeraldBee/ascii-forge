use std::{
    io::{self, Stdout, Write},
    panic::{set_hook, take_hook},
    time::Duration,
};

use crossterm::{
    cursor::{self, Hide, Show},
    event::{self, *},
    execute, queue,
    style::Print,
    terminal::{self, *},
    tty::IsTty,
};

use crate::{input::Input, prelude::*};

/// Struct that helps report what the terminal supports for you without you having to work with it yourself.
pub struct Supports {
    keyboard: bool,
}

impl Supports {
    /// Returns an error if the terminal doesn't support the kitty keyboard protocol.
    pub fn keyboard(&self) -> io::Result<()> {
        match self.keyboard {
            true => Ok(()),
            false => Err(io::Error::new(
                io::ErrorKind::Unsupported,
                "Terminal Type Not Supported: Kitty keyboard protocol not implemented.",
            )),
        }
    }
}

impl Default for Supports {
    fn default() -> Self {
        Self {
            keyboard: terminal::supports_keyboard_enhancement().expect("Terminal should be fine"),
        }
    }
}

#[derive(Default)]
pub struct Inline {
    active: bool,
    start: u16,
}

impl AsMut<Buffer> for Window {
    fn as_mut(&mut self) -> &mut Buffer {
        self.buffer_mut()
    }
}

/// The main window behind the application.
/// Represents the terminal window, allowing it to be used similar to a buffer,
/// but has extra event handling.
/**
```rust, no_run
use ascii_forge::prelude::*;

let mut window = Window::init()?;

render!(
    window,
    [
        (10, 10) => "Element Here!"
    ]
)
```
*/
pub struct Window {
    io: io::Stdout,
    buffers: [Buffer; 2],
    active_buffer: usize,
    events: Vec<Event>,

    // Input Helpers,
    mouse_pos: Vec2,

    // Inlining
    inline: Option<Inline>,

    // Input Support
    supports: Supports,
    input: Input,
}

impl Default for Window {
    fn default() -> Self {
        Self::init().expect("Init should have succeeded")
    }
}

impl Window {
    /// Creates a new window from the given stdout.
    /// Please prefer to use init as it will do all of the terminal init stuff.
    pub fn new(io: io::Stdout) -> io::Result<Self> {
        let supports = Supports::default();

        #[allow(unused_mut)]
        let mut input = Input::default();

        #[cfg(feature = "keyboard")]
        match supports.keyboard() {
            Ok(_) => {}
            Err(_) => input.no_kitty(),
        }

        Ok(Self {
            io,
            buffers: [Buffer::new(size()?), Buffer::new(size()?)],
            active_buffer: 0,
            events: vec![],

            mouse_pos: vec2(0, 0),

            inline: None,

            supports,
            input,
        })
    }

    /// Creates a new window built for inline using the given Stdout and height.
    pub fn new_inline(io: io::Stdout, height: u16) -> io::Result<Self> {
        let supports = Supports::default();

        #[allow(unused_mut)]
        let mut input = Input::default();

        #[cfg(feature = "keyboard")]
        match supports.keyboard() {
            Ok(_) => {}
            Err(_) => input.no_kitty(),
        }

        let size = vec2(size()?.0, height);
        Ok(Self {
            io,
            buffers: [Buffer::new(size), Buffer::new(size)],
            active_buffer: 0,
            events: vec![],

            mouse_pos: vec2(0, 0),

            inline: Some(Inline::default()),

            supports,
            input,
        })
    }

    /// Initializes a window that is prepared for inline rendering.
    /// Height is the number of columns that your terminal will need.
    pub fn init_inline(height: u16) -> io::Result<Self> {
        let stdout = io::stdout();

        assert!(stdout.is_tty());

        Window::new_inline(stdout, height)
    }

    /// Initializes the window, and returns a new Window for your use.
    pub fn init() -> io::Result<Self> {
        enable_raw_mode()?;

        let mut stdout = io::stdout();

        assert!(stdout.is_tty());

        execute!(
            stdout,
            EnterAlternateScreen,
            EnableMouseCapture,
            EnableFocusChange,
            Hide,
            DisableLineWrap,
        )?;

        // Enable keyboard enhancement if the terminal supports it.
        if Supports::default().keyboard().is_ok() {
            queue!(
                stdout,
                PushKeyboardEnhancementFlags(
                    KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES
                        | KeyboardEnhancementFlags::REPORT_ALL_KEYS_AS_ESCAPE_CODES
                        | KeyboardEnhancementFlags::REPORT_ALTERNATE_KEYS
                        | KeyboardEnhancementFlags::REPORT_EVENT_TYPES
                ),
            )?;
        }

        Window::new(stdout)
    }

    /// Returns the active Buffer, as a reference.
    pub fn buffer(&self) -> &Buffer {
        &self.buffers[self.active_buffer]
    }

    /// Returns the active Buffer, as a mutable reference.
    pub fn buffer_mut(&mut self) -> &mut Buffer {
        &mut self.buffers[self.active_buffer]
    }

    /// Swaps the buffers, clearing the old buffer. Used automatically by the window's update method.
    pub fn swap_buffers(&mut self) {
        self.active_buffer = 1 - self.active_buffer;
        self.buffers[self.active_buffer].clear();
    }

    /// Returns the current known size of the buffer's window.
    pub fn size(&self) -> Vec2 {
        self.buffer().size()
    }

    /// Restores the window to it's previous state from before the window's init method.
    /// If the window is inline, restore the inline render
    pub fn restore(&mut self) -> io::Result<()> {
        if self.supports().keyboard().is_ok() {
            queue!(self.io, PopKeyboardEnhancementFlags)?;
        }
        if self.inline.is_some() {
            execute!(
                self.io,
                DisableMouseCapture,
                DisableFocusChange,
                PopKeyboardEnhancementFlags,
            )?;

            println!();

            disable_raw_mode()?;
            Ok(())
        } else {
            execute!(
                self.io,
                PopKeyboardEnhancementFlags,
                LeaveAlternateScreen,
                DisableMouseCapture,
                DisableFocusChange,
                Show,
                EnableLineWrap,
            )?;

            disable_raw_mode()
        }
    }

    /// Renders the window to the screen. should really only be used by the update method, but if you need a custom system, you can use this.
    pub fn render(&mut self) -> io::Result<()> {
        if self.inline.is_some() {
            if !self.inline.as_ref().expect("Inline should be some").active {
                // Make room for the inline render
                print!("{}", "\n".repeat(self.buffer().size().y as usize));

                enable_raw_mode()?;

                execute!(
                    self.io,
                    EnableMouseCapture,
                    EnableFocusChange,
                    DisableLineWrap
                )?;

                if self.supports().keyboard().is_ok() {
                    execute!(
                        self.io,
                        PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::all())
                    )?;
                }

                let inline = self.inline.as_mut().expect("Inline should be some");

                inline.active = true;
                inline.start = cursor::position()?.1;
            }

            for (loc, cell) in
                self.buffers[1 - self.active_buffer].diff(&self.buffers[self.active_buffer])
            {
                queue!(
                    self.io,
                    cursor::MoveTo(
                        loc.x,
                        self.inline.as_ref().expect("Inline should be some").start
                            - self.buffers[self.active_buffer].size().y
                            + loc.y
                    ),
                    Print(cell),
                )?;
            }
        } else {
            for (loc, cell) in
                self.buffers[1 - self.active_buffer].diff(&self.buffers[self.active_buffer])
            {
                queue!(self.io, cursor::MoveTo(loc.x, loc.y), Print(cell))?;
            }
        }
        Ok(())
    }

    /// Handles events, and renders the screen.
    pub fn update(&mut self, poll: Duration) -> io::Result<()> {
        let cursor_pos = cursor::position()?;

        // Render Window
        self.render()?;

        self.swap_buffers();

        queue!(self.io, cursor::MoveTo(cursor_pos.0, cursor_pos.1))?;

        // Flush Render To Stdout
        self.io.flush()?;

        // Poll For Events
        self.handle_event(poll)?;

        Ok(())
    }

    /// Handles events. Used automatically by the update method, so no need to use it unless update is being used.
    pub fn handle_event(&mut self, poll: Duration) -> io::Result<()> {
        self.events = vec![];

        self.input.update();

        if event::poll(poll)? {
            // Get all queued events
            while event::poll(Duration::ZERO)? {
                let event = event::read()?;

                match event {
                    Event::Resize(width, height) => {
                        if self.inline.is_none() {
                            self.buffers =
                                [Buffer::new((width, height)), Buffer::new((width, height))];
                        }
                    }
                    Event::Mouse(MouseEvent { column, row, .. }) => {
                        self.mouse_pos = vec2(column, row)
                    }
                    _ => {}
                }

                self.input.register_event(event.clone());

                self.events.push(event);
            }
        }

        Ok(())
    }

    pub fn input(&self) -> &Input {
        &self.input
    }

    pub fn input_mut(&mut self) -> &mut Input {
        &mut self.input
    }

    pub fn mouse_pos(&self) -> Vec2 {
        self.mouse_pos
    }

    /// Returns the current event for the frame, as a reference.
    pub fn events(&self) -> &Vec<Event> {
        &self.events
    }

    /// Returns true if the mouse cursor is hovering the given rect.
    pub fn hover<I: Into<Vec2>>(&self, loc: I, size: I) -> io::Result<bool> {
        let loc = loc.into();
        let size = size.into();

        let pos: Vec2 = self.mouse_pos();

        Ok(pos.x <= loc.x + size.x && pos.x >= loc.x && pos.y <= loc.y + size.y && pos.y >= loc.y)
    }

    pub fn io(&mut self) -> &mut Stdout {
        &mut self.io
    }

    pub fn supports(&self) -> &Supports {
        &self.supports
    }
}

/// Enables a panic hook to help you terminal still look pretty.
pub fn handle_panics() {
    let original_hook = take_hook();
    set_hook(Box::new(move |e| {
        Window::new(io::stdout())
            .expect("Window should have created for panic")
            .restore()
            .expect("Window should have exited for panic");
        original_hook(e);
    }))
}

impl Drop for Window {
    fn drop(&mut self) {
        self.restore().expect("Restoration should have succeded");
    }
}
