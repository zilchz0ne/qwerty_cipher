use crate::typing;
use crate::cipher::Cipher;

pub fn frontend(args: Vec<String>) {
    let typing = typing::Typing::new();

    if args.len() < 3 {
        println!("Usage: qwerty_cipher encrypt -l [left_hand_shift] -r [right_hand_shift] [plaintext]");
        return;
    }

    let mut pos_right_arg = 4;

    let left: i32 = match args[2].as_str() {
        "-l" => args[3].parse().expect("Error: not an integer"),
        _ => {
            pos_right_arg = 2;
            0
        },
    };

    let right: i32 = match args[pos_right_arg].as_str() {
        "-r" => args[pos_right_arg + 1].parse().expect("Error: not an integer"),
        _ => 0,
    };

    let text = &args[pos_right_arg + 2];

    match args[1].as_str() {
        "encrypt" => {
            let ciphertext = typing.encrypt(text, left, right);
            println!("{}", ciphertext);
            return;
        }
        "decrypt" => {
            println!("Error: Under Construction. It's Hard!");
            return;
        }
        _ => {
            println!("Error: Use encrypt or decrypt");
            return;
        }
    }

}
