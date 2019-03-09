use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
#[repr(u16)]
pub enum Key {
    W = 0,
    A = 1,
    S = 2,
    D = 3,
}

impl Key {
    pub fn from_str(code: &str) -> Option<Key> {
        match code {
            "KeyW" => Some(Key::W),
            "KeyA" => Some(Key::A),
            "KeyS" => Some(Key::S),
            "KeyD" => Some(Key::D),
            _ => None,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct KeyboardState {
    pressed_keys: HashMap<Key, ()>,
}

impl KeyboardState {
    pub fn new() -> Self {
        KeyboardState {
            pressed_keys: HashMap::new(),
        }
    }
    pub fn set_key_down(&mut self, k: Key) {
        if !self.pressed_keys.contains_key(&k) {
            self.pressed_keys.insert(k, ());
        }
    }
    pub fn set_key_up(&mut self, k: Key) {
        if self.pressed_keys.contains_key(&k) {
            self.pressed_keys.remove(&k);
        }
    }
    pub fn is_key_down(&self, k: Key) -> bool {
        self.pressed_keys.contains_key(&k)
    }

    pub fn is_key_up(&self, k: Key) -> bool {
        !self.pressed_keys.contains_key(&k)
    }

    pub fn clear(&mut self) {
        self.pressed_keys.clear();
    }

    pub fn copy_from(&mut self, other: &KeyboardState) {
        self.clear();
        for (k, _) in other.pressed_keys.iter() {
            self.pressed_keys.insert(*k, ());
        }
    }
}
