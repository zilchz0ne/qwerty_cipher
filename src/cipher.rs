use crate::typing;

pub trait Cipher {
    // fn get_key(&self, finger: (i32, i32)) -> Option<char>;
    // fn get_finger(&self, key: char) -> Option<(i32, i32)>;
    fn left_hand_shift(&self, text: &String, shift: i32) -> String; // positive shift means shifting right
    fn right_hand_shift(&self, text: &String, shift: i32) -> String;
}

impl Cipher for typing::Typing {
    fn left_hand_shift(&self, text: &String, shift: i32) -> String {
        let mut cipher = String::new();
        for c in text.chars() {
            let finger = match self.get_finger(&c) {
                Some(thing) => { thing },
                None => { 
                    &(4, 0)
                },
            };

            let border = match finger.0 {
                0 => self.right_border.0.1,
                1 => self.right_border.1.1,
                2 => self.right_border.2.1,
                _ => self.right_border.3.1,
            };

            let shifted: i32 = if finger.1 < border {
                finger.1 + shift
            } else {
                finger.1
            };

            match self.get_key(&(finger.0, shifted)) {
                Some(thing) => { cipher.push(*thing); },
                None => { cipher.push('~') },
            };
        }
        cipher
    }

fn right_hand_shift(&self, text: &String, shift: i32) -> String {
        let mut cipher = String::new();
        for c in text.chars() {
            let finger = match self.get_finger(&c) {
                Some(thing) => { thing },
                None => { 
                    &(4, 0)
                },
            };

            let border = match finger.0 {
                0 => self.right_border.0.1,
                1 => self.right_border.1.1,
                2 => self.right_border.2.1,
                _ => self.right_border.3.1,
            };

            let shifted: i32 = if finger.1 >= border {
                finger.1 + shift
            } else {
                finger.1
            };

            match self.get_key(&(finger.0, shifted)) {
                Some(thing) => { cipher.push(*thing); },
                None => { cipher.push('~') },
            };
        }
        cipher
    }
}
