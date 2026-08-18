use std::collections::HashMap;

pub struct Keyboard {
    map: HashMap<u8, (i32, i32)>,
}

impl Keyboard {
    pub fn new() -> Self {
        let mut map = HashMap::<u8, (i32, i32)>::new();

        map.insert(b'1', (0, 0));
        map.insert(b'2', (0, 1));
        map.insert(b'3', (0, 2));
        map.insert(b'4', (0, 3));
        map.insert(b'5', (0, 4));
        map.insert(b'6', (0, 5));
        map.insert(b'7', (0, 6));
        map.insert(b'8', (0, 7));
        map.insert(b'9', (0, 8));
        map.insert(b'0', (0, 9));
        map.insert(b'-', (0, 10));
        map.insert(b'=', (0, 11));

        map.insert(b'q', (1, 0));
        map.insert(b'w', (1, 1));
        map.insert(b'e', (1, 2));
        map.insert(b'r', (1, 3));
        map.insert(b't', (1, 4));
        map.insert(b'y', (1, 5));
        map.insert(b'u', (1, 6));
        map.insert(b'i', (1, 7));
        map.insert(b'o', (1, 8));
        map.insert(b'p', (1, 9));
        map.insert(b'[', (1, 10));
        map.insert(b']', (1, 11));

        map.insert(b'a', (2, 0));
        map.insert(b's', (2, 1));
        map.insert(b'd', (2, 2));
        map.insert(b'f', (2, 3));
        map.insert(b'g', (2, 4));
        map.insert(b'h', (2, 5));
        map.insert(b'j', (2, 6));
        map.insert(b'k', (2, 7));
        map.insert(b'l', (2, 8));
        map.insert(b';', (2, 9));
        map.insert(b'\'', (2, 10));
        map.insert(b'\\', (2, 11));

        map.insert(b'z', (3, 0));
        map.insert(b'x', (3, 1));
        map.insert(b'c', (3, 2));
        map.insert(b'v', (3, 3));
        map.insert(b'b', (3, 4));
        map.insert(b'n', (3, 5));
        map.insert(b'm', (3, 6));
        map.insert(b',', (3, 7));
        map.insert(b'.', (3, 8));
        map.insert(b'/', (3, 9));

        map.insert(b'~', (0, -1));
        map.insert(b' ', (4, 0));

        Self { map: map }
    }

    pub fn get(&self, key: &u8) -> Option<&(i32, i32)> {
        self.map.get(key)
    }
}

pub struct Finger {
    map: HashMap<(i32, i32), u8>,
}

impl Finger {
    pub fn new() -> Self {
        let mut map = HashMap::<(i32, i32), u8>::new();

        map.insert((0, 0), b'1');
        map.insert((0, 1), b'2');
        map.insert((0, 2), b'3');
        map.insert((0, 3), b'4');
        map.insert((0, 4), b'5');
        map.insert((0, 5), b'6');
        map.insert((0, 6), b'7');
        map.insert((0, 7), b'8');
        map.insert((0, 8), b'9');
        map.insert((0, 9), b'0');
        map.insert((0, 10), b'-');
        map.insert((0, 11), b'=');

        map.insert((1, 0), b'q');
        map.insert((1, 1), b'w');
        map.insert((1, 2), b'e');
        map.insert((1, 3), b'r');
        map.insert((1, 4), b't');
        map.insert((1, 5), b'y');
        map.insert((1, 6), b'u');
        map.insert((1, 7), b'i');
        map.insert((1, 8), b'o');
        map.insert((1, 9), b'p');
        map.insert((1, 10), b'[');
        map.insert((1, 11), b']');

        map.insert((2, 0), b'a');
        map.insert((2, 1), b's');
        map.insert((2, 2), b'd');
        map.insert((2, 3), b'f');
        map.insert((2, 4), b'g');
        map.insert((2, 5), b'h');
        map.insert((2, 6), b'j');
        map.insert((2, 7), b'k');
        map.insert((2, 8), b'l');
        map.insert((2, 9), b';');
        map.insert((2, 10), b'\'');
        map.insert((2, 11), b'\\');

        map.insert((3, 0), b'z');
        map.insert((3, 1), b'x');
        map.insert((3, 2), b'c');
        map.insert((3, 3), b'v');
        map.insert((3, 4), b'b');
        map.insert((3, 5), b'n');
        map.insert((3, 6), b'm');
        map.insert((3, 7), b',');
        map.insert((3, 8), b'.');
        map.insert((3, 9), b'/');

        map.insert((0, -1), b'~');
        map.insert((4, 0), b' ');

        Self { map: map }
    }

    pub fn get(&self, press: &(i32, i32)) -> Option<&u8> {
        self.map.get(press)
    }
}



pub struct Typing {
    keyboard: Keyboard,
    finger: Finger,
    right_hand_border: ((i32, i32), (i32, i32), (i32, i32), (i32, i32), (i32, i32)),
}

impl Typing {
    pub fn new() -> Self {
        let keyboard = Keyboard::new();
        let finger = Finger::new();

        Self {
            keyboard: keyboard,
            finger: finger,
            right_hand_border: ((0, 6), (1, 5), (2, 5), (3, 4), (4, 0)), //('7', 'y', 'h', 'b', ' ')
        }
    }

    pub fn get_press(&self, key: &u8) -> Option<&(i32, i32)> {
        self.keyboard.get(key)
    }

    pub fn get_key(&self, press: &(i32, i32)) -> Option<&u8> {
        self.finger.get(press)
    }

    pub fn is_right_hand_press(&self, press: &(i32, i32)) -> bool {
        if self.get_key(press) == None {
            return false;
        }

        let border = match press.0 {
            0 => self.right_hand_border.0.1,
            1 => self.right_hand_border.1.1,
            2 => self.right_hand_border.2.1,
            3 => self.right_hand_border.3.1,
            4 => self.right_hand_border.4.1,
            _ => { return false; },
        };

        if press.1 >= border {
            true
        } else {
            false
        }
    }

    pub fn is_left_hand_press(&self, press: &(i32, i32)) -> bool {
        if self.get_key(press) == None {
            return false;
        }

        !self.is_right_hand_press(press)
    }

    pub fn is_right_hand_key(&self, key: &u8) -> bool {
        match self.get_press(key) {
            Some(thing) => self.is_right_hand_press(thing),
            None => false,
        }
    }

    pub fn is_left_hand_key(&self, key: &u8) -> bool {
        match self.get_press(key) {
            Some(thing) => self.is_left_hand_press(thing),
            None => false,
        }
    }

    pub fn shift_press(&self, press: &(i32, i32), shift: i32) -> Option<(i32, i32)> {
        if *press == (4, 0) || *press == (0, -1) {
            return Some(*press);
        }
        
        let shifted = (press.0, press.1 + shift);
        
        if self.get_key(&shifted) == None {
            None
        } else {
            Some(shifted)
        }
    }

    pub fn shift_key(&self, key: &u8, shift: i32) -> Option<u8> { // shift: i32 < 0 is left shift
        let press = match self.get_press(key) {
            Some(thing) => { *thing }
            None => { return None; }
        };

        match self.shift_press(&press, shift) {
            Some(thing) => { self.get_key(&thing).copied() }
            None => { return None; }
        }
    }
}
