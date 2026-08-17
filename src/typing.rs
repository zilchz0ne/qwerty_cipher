use std::collections::HashMap;

pub struct Keyboard {
    keymap: HashMap<char, (i32, i32)>,
}

pub struct Finger {
    fingermap: HashMap<(i32, i32), char>,
}

impl Keyboard {
    pub fn new() -> Self {
        let mut keymap = HashMap::<char, (i32, i32)>::new();

        keymap.insert('1', (0, 0));
        keymap.insert('2', (0, 1));
        keymap.insert('3', (0, 2));
        keymap.insert('4', (0, 3));
        keymap.insert('5', (0, 4));
        keymap.insert('6', (0, 5));
        keymap.insert('7', (0, 6));
        keymap.insert('8', (0, 7));
        keymap.insert('9', (0, 8));
        keymap.insert('0', (0, 9));
        keymap.insert('-', (0, 10));
        keymap.insert('=', (0, 11));

        keymap.insert('q', (1, 0));
        keymap.insert('w', (1, 1));
        keymap.insert('e', (1, 2));
        keymap.insert('r', (1, 3));
        keymap.insert('t', (1, 4));
        keymap.insert('y', (1, 5));
        keymap.insert('u', (1, 6));
        keymap.insert('i', (1, 7));
        keymap.insert('o', (1, 8));
        keymap.insert('p', (1, 9));
        keymap.insert('[', (1, 10));
        keymap.insert(']', (1, 11));

        keymap.insert('a', (2, 0));
        keymap.insert('s', (2, 1));
        keymap.insert('d', (2, 2));
        keymap.insert('f', (2, 3));
        keymap.insert('g', (2, 4));
        keymap.insert('h', (2, 5));
        keymap.insert('j', (2, 6));
        keymap.insert('k', (2, 7));
        keymap.insert('l', (2, 8));
        keymap.insert(';', (2, 9));
        keymap.insert('\'', (2, 10));
        keymap.insert('\\', (2, 11));

        keymap.insert('z', (3, 0));
        keymap.insert('x', (3, 1));
        keymap.insert('c', (3, 2));
        keymap.insert('v', (3, 3));
        keymap.insert('b', (3, 4));
        keymap.insert('n', (3, 5));
        keymap.insert('m', (3, 6));
        keymap.insert(',', (3, 7));
        keymap.insert('.', (3, 8));
        keymap.insert('/', (3, 9));

        Keyboard { keymap: keymap }
    }

    pub fn get(&self, key: &char) -> Option<&(i32, i32)> {
        self.keymap.get(key)
    }
}

impl Finger {
    pub fn new() -> Self {
        let mut fingermap = HashMap::<(i32, i32), char>::new();

        fingermap.insert((0, 0), '1');
        fingermap.insert((0, 1), '2');
        fingermap.insert((0, 2), '3');
        fingermap.insert((0, 3), '4');
        fingermap.insert((0, 4), '5');
        fingermap.insert((0, 5), '6');
        fingermap.insert((0, 6), '7');
        fingermap.insert((0, 7), '8');
        fingermap.insert((0, 8), '9');
        fingermap.insert((0, 9), '0');
        fingermap.insert((0, 10), '-');
        fingermap.insert((0, 11), '=');

        fingermap.insert((1, 0), 'q');
        fingermap.insert((1, 1), 'w');
        fingermap.insert((1, 2), 'e');
        fingermap.insert((1, 3), 'r');
        fingermap.insert((1, 4), 't');
        fingermap.insert((1, 5), 'y');
        fingermap.insert((1, 6), 'u');
        fingermap.insert((1, 7), 'i');
        fingermap.insert((1, 8), 'o');
        fingermap.insert((1, 9), 'p');
        fingermap.insert((1, 10), '[');
        fingermap.insert((1, 11), ']');

        fingermap.insert((2, 0), 'a');
        fingermap.insert((2, 1), 's');
        fingermap.insert((2, 2), 'd');
        fingermap.insert((2, 3), 'f');
        fingermap.insert((2, 4), 'g');
        fingermap.insert((2, 5), 'h');
        fingermap.insert((2, 6), 'j');
        fingermap.insert((2, 7), 'k');
        fingermap.insert((2, 8), 'l');
        fingermap.insert((2, 9), ';');
        fingermap.insert((2, 10), '\'');
        fingermap.insert((2, 11), '\\');

        fingermap.insert((3, 0), 'z');
        fingermap.insert((3, 1), 'x');
        fingermap.insert((3, 2), 'c');
        fingermap.insert((3, 3), 'v');
        fingermap.insert((3, 4), 'b');
        fingermap.insert((3, 5), 'n');
        fingermap.insert((3, 6), 'm');
        fingermap.insert((3, 7), ',');
        fingermap.insert((3, 8), '.');
        fingermap.insert((3, 9), '/');

        Finger { fingermap: fingermap }
    }

    pub fn get(&self, finger: &(i32, i32)) -> Option<&char> {
        self.fingermap.get(finger)
    }
}


pub struct Typing {
    keyboard: Keyboard,
    finger: Finger,
    pub right_border: ((i32, i32), (i32, i32), (i32, i32), (i32, i32)),
}

impl Typing {
    pub fn new() -> Self {
        let keyboard = Keyboard::new();
        let finger = Finger::new();

        Typing {
            keyboard: keyboard,
            finger: finger,
            right_border: ((0, 6), (1, 5), (2, 5), (3, 4)), //('7', 'y', 'h', 'b')
        }
    }

    pub fn get_finger(&self, key: &char) -> Option<&(i32, i32)> {
        self.keyboard.get(key)
    }

    pub fn get_key(&self, finger: &(i32, i32)) -> Option<&char> {
        self.finger.get(finger)
    }
}
