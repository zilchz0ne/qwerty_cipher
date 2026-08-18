use crate::typing;

pub trait Cipher {
    fn encrypt(&self, plaintext: &String, left_hand_shift: i32, right_hand_shift: i32) -> String;
}

impl Cipher for typing::Typing {
    fn encrypt(&self, plaintext: &String, left_hand_shift: i32, right_hand_shift: i32) -> String {
        let mut ciphertext = String::new();

        for character in plaintext.bytes() {
            let shifted = if self.is_right_hand_key(&character) {
                match self.shift_key(&character, right_hand_shift) {
                    Some(thing) => thing,
                    None => b'~',
                }
            } else if self.is_left_hand_key(&character) {
                match self.shift_key(&character, left_hand_shift) {
                    Some(thing) => thing,
                    None => b'~',
                }
            } else {
                b'~'
            };

            ciphertext.push(char::from(shifted));
        }

        ciphertext
    }
}
