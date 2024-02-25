use std::io;

use crossterm::queue;

use crate::prelude::*;

/// The trait that should be implemented for any input protocol.
pub trait InputTrait {
    /// The function that should be used to setup the terminal.
    fn setup(&mut self, supports: &Supports) -> io::Result<()>;

    /// The function that should be used to update the input for the terminal.
    /// Used to clear the frame based input vectors.
    fn update(&mut self);

    /// This function should be used to register the events from the terminal.
    fn register_event(&mut self, event: Event);

    /// Registers the given key event to the input system.
    fn register_key(&mut self, key_event: KeyEvent);

    /// Registers the mouse event to the input system.
    fn register_mouse(&mut self, mouse_event: MouseEvent);
}

/// The input type for your regular keyboard terminal.
#[derive(Default, Debug)]
pub struct Input {
    keys: Vec<KeyEvent>,

    /// All of the mouse events
    just_pressed_mouse: Vec<MouseButton>,
    mouse: Vec<MouseButton>,
    just_released_mouse: Vec<MouseButton>,

    /// The scroll value from the last frame.
    scroll: u16,
}

impl InputTrait for Input {
    fn setup(&mut self, _supports: &Supports) -> io::Result<()> {
        Ok(())
    }

    fn update(&mut self) {
        self.keys.clear();

        self.just_pressed_mouse.clear();
        self.just_released_mouse.clear();
    }

    fn register_event(&mut self, event: Event) {
        match event {
            Event::Key(key_event) => self.register_key(key_event),
            Event::Mouse(mouse_event) => self.register_mouse(mouse_event),
            _ => {}
        }
    }

    fn register_key(&mut self, key_event: KeyEvent) {
        self.keys.push(key_event);
    }

    fn register_mouse(&mut self, mouse_event: MouseEvent) {
        match mouse_event.kind {
            MouseEventKind::Down(button) => {
                self.just_pressed_mouse.push(button);
                self.mouse.push(button);
            }
            MouseEventKind::Up(button) => {
                self.just_released_mouse.push(button);
                self.mouse.retain(|x| *x != button)
            }
            MouseEventKind::ScrollDown => {
                self.scroll += 1;
            }
            MouseEventKind::ScrollUp => {
                self.scroll -= 1;
            }
            _ => {}
        }
    }
}

impl Input {
    /// Returns true if the provided KeyCode was pressed this frame.
    pub fn code(&self, code: KeyCode) -> bool {
        self.keys.iter().any(|x| x.code == code)
    }

    /// Returns true if the given KeyEvent was enacted this frame.
    pub fn pressed(&self, key_event: KeyEvent) -> bool {
        self.keys.contains(&key_event)
    }

    /// Returns true if the given mouse button was just pressed.
    pub fn mouse_just_pressed(&self, button: &MouseButton) -> bool {
        self.just_pressed_mouse.contains(button)
    }

    /// Returns true if the given mouse button is currently down.
    pub fn mouse_pressed(&self, button: &MouseButton) -> bool {
        self.mouse.contains(button)
    }

    /// Returns true if the given mouse button was just released.
    pub fn mouse_just_released(&self, button: &MouseButton) -> bool {
        self.just_released_mouse.contains(button)
    }
}

/// The input type for terminals with support for the kitty keyboard protocol
#[derive(Default, Debug)]
pub struct KittyInput {
    /// All of the key events
    just_pressed: Vec<(KeyCode, KeyModifiers)>,

    keys: Vec<(KeyCode, KeyModifiers)>,

    just_released: Vec<(KeyCode, KeyModifiers)>,

    /// All of the mouse events
    just_pressed_mouse: Vec<MouseButton>,
    mouse: Vec<MouseButton>,
    just_released_mouse: Vec<MouseButton>,

    /// The scroll value from the last frame.
    scroll: u16,
}

impl InputTrait for KittyInput {
    fn setup(&mut self, supports: &Supports) -> io::Result<()> {
        match supports.keyboard() {
            Ok(_) => {
                queue!(
                    io::stdout(),
                    PushKeyboardEnhancementFlags(
                        KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES
                            | KeyboardEnhancementFlags::REPORT_ALL_KEYS_AS_ESCAPE_CODES
                            | KeyboardEnhancementFlags::REPORT_ALTERNATE_KEYS
                            | KeyboardEnhancementFlags::REPORT_EVENT_TYPES
                    ),
                )?;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }
    fn update(&mut self) {
        self.just_pressed.clear();
        self.just_released.clear();

        self.just_pressed_mouse.clear();
        self.just_released_mouse.clear();
    }

    fn register_event(&mut self, event: Event) {
        match event {
            Event::Key(key_event) => self.register_key(key_event),
            Event::Mouse(mouse_event) => self.register_mouse(mouse_event),
            _ => {}
        }
    }

    fn register_key(&mut self, key_event: KeyEvent) {
        match key_event.kind {
            KeyEventKind::Press => {
                self.just_pressed
                    .push((key_event.code, key_event.modifiers));
                self.keys.push((key_event.code, key_event.modifiers));
            }
            KeyEventKind::Release => {
                self.just_released
                    .push((key_event.code, key_event.modifiers));

                // Remove the key_code from the pressed_keys vec.
                self.keys.retain(|x| x.0 != key_event.code);
            }
            _ => {}
        }
    }

    fn register_mouse(&mut self, mouse_event: MouseEvent) {
        match mouse_event.kind {
            MouseEventKind::Down(button) => {
                self.just_pressed_mouse.push(button);
                self.mouse.push(button);
            }
            MouseEventKind::Up(button) => {
                self.just_released_mouse.push(button);
                self.mouse.retain(|x| *x != button)
            }
            MouseEventKind::ScrollDown => {
                self.scroll += 1;
            }
            MouseEventKind::ScrollUp => {
                self.scroll -= 1;
            }
            _ => {}
        }
    }
}

impl KittyInput {
    pub fn just_pressed_mod(&self, code: KeyCode, modifier: KeyModifiers) -> bool {
        self.just_pressed.contains(&(code, modifier))
    }

    pub fn just_pressed(&self, code: KeyCode) -> bool {
        self.just_pressed.iter().any(|x| x.0 == code)
    }

    pub fn pressed_mod(&self, code: KeyCode, modifier: KeyModifiers) -> bool {
        self.keys.contains(&(code, modifier))
    }

    pub fn pressed(&self, code: KeyCode) -> bool {
        self.keys.iter().any(|x| x.0 == code)
    }

    pub fn just_released(&self, code: KeyCode) -> bool {
        self.just_released.iter().any(|x| x.0 == code)
    }

    pub fn just_released_mod(&self, code: KeyCode, modifier: KeyModifiers) -> bool {
        self.just_released.contains(&(code, modifier))
    }

    pub fn mouse_just_pressed(&self, button: &MouseButton) -> bool {
        self.just_pressed_mouse.contains(button)
    }

    pub fn mouse_pressed(&self, button: &MouseButton) -> bool {
        self.mouse.contains(button)
    }

    pub fn mouse_just_released(&self, button: &MouseButton) -> bool {
        self.just_released_mouse.contains(button)
    }
}
