use crate::prelude::*;

#[derive(Default, Debug)]
pub struct Input {
    /// All of the key events
    #[cfg(feature = "keyboard")]
    just_pressed: Vec<(KeyCode, KeyModifiers)>,

    #[cfg(feature = "keyboard")]
    keys: Vec<(KeyCode, KeyModifiers)>,

    #[cfg(not(feature = "keyboard"))]
    keys: Vec<KeyEvent>,

    #[cfg(feature = "keyboard")]
    just_released: Vec<(KeyCode, KeyModifiers)>,

    /// All of the mouse events
    just_pressed_mouse: Vec<MouseButton>,
    mouse: Vec<MouseButton>,
    just_released_mouse: Vec<MouseButton>,

    /// The scroll value from the last frame.
    scroll: u16,

    #[cfg(feature = "keyboard")]
    not_kitty: bool,
}

impl Input {
    #[cfg(feature = "keyboard")]
    pub fn no_kitty(&mut self) {
        self.not_kitty = true;
    }

    /// Clears just used events.
    pub fn update(&mut self) {
        #[cfg(feature = "keyboard")]
        self.just_pressed.clear();
        #[cfg(feature = "keyboard")]
        self.just_released.clear();

        self.just_pressed_mouse.clear();
        self.just_released_mouse.clear();

        #[cfg(not(feature = "keyboard"))]
        self.keys.clear();

        #[cfg(feature = "keyboard")]
        if self.not_kitty {
            self.keys.clear();
        }
    }

    pub fn register_event(&mut self, event: Event) {
        match event {
            Event::Key(key_event) => self.register_key(key_event),
            Event::Mouse(mouse_event) => self.register_mouse(mouse_event),
            _ => {}
        }
    }

    pub fn register_key(&mut self, key_event: KeyEvent) {
        #[cfg(feature = "keyboard")]
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

        #[cfg(not(feature = "keyboard"))]
        self.keys.push(key_event);
    }

    pub fn register_mouse(&mut self, mouse_event: MouseEvent) {
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

    #[cfg(feature = "keyboard")]
    pub fn just_pressed_mod(&self, code: KeyCode, modifier: KeyModifiers) -> bool {
        self.just_pressed.contains(&(code, modifier))
    }

    #[cfg(feature = "keyboard")]
    pub fn just_pressed(&self, code: KeyCode) -> bool {
        self.just_pressed.iter().any(|x| x.0 == code)
    }

    #[cfg(feature = "keyboard")]
    pub fn pressed_mod(&self, code: KeyCode, modifier: KeyModifiers) -> bool {
        self.keys.contains(&(code, modifier))
    }

    #[cfg(feature = "keyboard")]
    pub fn pressed(&self, code: KeyCode) -> bool {
        self.keys.iter().any(|x| x.0 == code)
    }

    #[cfg(feature = "keyboard")]
    pub fn just_released(&self, code: KeyCode) -> bool {
        self.just_released.iter().any(|x| x.0 == code)
    }

    #[cfg(feature = "keyboard")]
    pub fn just_released_mod(&self, code: KeyCode, modifier: KeyModifiers) -> bool {
        self.just_released.contains(&(code, modifier))
    }

    #[cfg(not(feature = "keyboard"))]
    pub fn pressed(&self, event: &KeyEvent) -> bool {
        self.keys.contains(event)
    }

    #[cfg(not(feature = "keyboard"))]
    pub fn code(&self, code: KeyCode) -> bool {
        self.keys.contains(&KeyEvent::new(code, KeyModifiers::NONE))
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
