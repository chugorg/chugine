use glfw::{Key, Action};

pub struct InputSystem {
    just_pressed: Vec<Key>,
    pressed: Vec<Key>,
    just_released: Vec<Key>,
}

impl InputSystem {
    pub fn is_just_pressed(&self, key: Key) -> bool {
        self.just_pressed.contains(&key)
    }

    pub fn is_pressed(&self, key: Key) -> bool {
        self.pressed.contains(&key)
    }

    pub fn is_just_released(&self, key: Key) -> bool {
        self.just_released.contains(&key)
    }

    pub fn clear(&mut self) {
        self.just_pressed.clear();
        self.just_released.clear();
    }

    pub fn new() -> Self {
        Self {
            just_pressed: Vec::new(),
            pressed: Vec::new(),
            just_released: Vec::new(),
        }
    }

    pub fn update(&mut self, key: Key, action: Action) {
        match action {
            Action::Press => {
                self.just_pressed.push(key);
                self.pressed.push(key);
            }

            Action::Repeat => {}

            Action::Release => {
                self.just_released.push(key);
                let position = self.pressed.iter().position(|p_key| *p_key == key);
                if let Some(position) = position {
                    self.pressed.remove(position);
                }
            }
        }
    }
}
