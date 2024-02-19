use std::{
    io,
    panic::{set_hook, take_hook},
    time::Duration,
};

use crossterm::{
    cursor::{Hide, Show},
    event::{self, *},
    execute,
    terminal::*,
};

use crate::{buffer::Buffer, math::Vec2};

pub struct Window {
    io: io::Stdout,
    buffer: Buffer,
    event: Option<Event>,
}

impl Default for Window {
    fn default() -> Self {
        Self::init().expect("Init should have succeeded")
    }
}

impl Window {
    pub fn new(io: io::Stdout) -> io::Result<Self> {
        Ok(Self {
            io,
            buffer: Buffer::new(size()?.into()),
            event: None,
        })
    }

    pub fn init() -> io::Result<Self> {
        let mut stdout = io::stdout();

        enable_raw_mode()?;
        execute!(
            stdout,
            EnterAlternateScreen,
            EnableMouseCapture,
            EnableFocusChange,
            Hide,
            DisableLineWrap,
        )?;

        Window::new(stdout)
    }

    pub fn buffer(&mut self) -> &mut Buffer {
        &mut self.buffer
    }

    pub fn size(&self) -> Vec2 {
        self.buffer.size()
    }

    pub fn restore(mut self) -> io::Result<()> {
        disable_raw_mode()?;
        execute!(
            self.io,
            LeaveAlternateScreen,
            DisableMouseCapture,
            DisableFocusChange,
            Show,
            EnableLineWrap,
        )?;

        Ok(())
    }

    pub fn update(&mut self) -> io::Result<()> {
        // Render Window
        self.buffer.io_render(&mut self.io)?;

        // Poll For Events
        self.handle_event()?;

        Ok(())
    }

    pub fn handle_event(&mut self) -> io::Result<()> {
        self.event = None;

        if event::poll(Duration::ZERO)? {
            self.event = Some(event::read()?);
        }

        Ok(())
    }

    pub fn event(&self) -> &Option<Event> {
        &self.event
    }

    pub fn key_code(&self, key: KeyCode) -> bool {
        self.key(KeyEvent::new(key, KeyModifiers::NONE))
    }

    pub fn key(&self, key: KeyEvent) -> bool {
        self.event == Some(Event::Key(key))
    }
}

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
