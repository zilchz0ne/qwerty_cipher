use crate::typing;

pub trait Cipher {
    fn gen_cipherkey(&self, plaintext: &String) -> String;
    fn encrypt(&self, plaintext: &String, left_hand_shift: i32, right_hand_shift: i32) -> String;
    fn decrypt(&self, ciphertext: &String, left_hand_shift: i32, right_hand_shift: i32, cipherkey: &String) -> String;
}

impl Cipher for typing::Typing {
    fn gen_cipherkey(&self, plaintext: &String) -> String {
        let mut cipherkey = String::new();

        for character in plaintext.bytes() {
            if self.is_right_hand_key(&character) {
                cipherkey.push('1');
            } else {
                cipherkey.push('0');
            }
        }

        cipherkey
    }

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

    fn decrypt(&self, ciphertext: &String, left_hand_shift: i32, right_hand_shift: i32, cipherkey: &String) -> String {
        let mut plaintext = String::new();

        for (index, character) in ciphertext.bytes().enumerate() {
            let shifted = if cipherkey.as_bytes()[index] == b'1' {
                match self.shift_key(&character, -1 * right_hand_shift) {
                    Some(thing) => {
                        if self.is_right_hand_key(&thing) {
                            thing
                        } else {
                            b'~'
                        }
                    }
                    None => b'~',
                }
            } else if cipherkey.as_bytes()[index] == b'0' {
                match self.shift_key(&character, -1 * left_hand_shift) {
                    Some(thing) => {
                        if self.is_left_hand_key(&thing) {
                            thing
                        } else {
                            b'~'
                        }
                    }
                    None => b'~',
                }
            } else {
                b'~'
            };

            plaintext.push(char::from(shifted));
        }

        plaintext
    }
}





