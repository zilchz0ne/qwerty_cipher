use crate::typing;
use crate::cipher::Cipher;

pub fn frontend(args: Vec<String>) {
    let typing = typing::Typing::new();

    if args.len() < 7 {
        println!("Usage: qwerty_cipher [encrypt/decrypt] -l [left_hand_shift] -r [right_hand_shift] -k(optional) [cipherkey] [plaintext/ciphertext]");
        return;
    }

    let left: i32 = match args[2].as_str() {
        "-l" => args[3].parse().expect("Error: not an integer"),
        _ => {
            println!("Error: Use -l as 3rd argument");
            return;
        },
    };

    let right: i32 = match args[4].as_str() {
        "-r" => args[5].parse().expect("Error: not an integer"),
        _ => {
            println!("Error: Use -r as 5rd argument");
            return;
        },
    };

    let mut arg_text_index = 8;
    let cipherkey = match args[6].as_str() {
        "-k" => &args[7],
        _ => {
            arg_text_index = 6;
            &"n".to_string()
        }
    };

    let text = &args[arg_text_index];

    match args[1].as_str() {
        "encrypt" => {
            let ciphertext = typing.encrypt(text, left, right);
            let cipherkey = typing.gen_cipherkey(text);
            println!("ciphertext = {}", ciphertext);
            println!("cipherkey = {}", cipherkey);
            return;
        }
        "decrypt" => {
            if *cipherkey == "n" {
                let plaintext_assuming_left_hand = typing.decrypt(text, left, right, &"0".repeat(text.len()));
                println!("plaintext left = {}", plaintext_assuming_left_hand);
                let plaintext_assuming_right_hand = typing.decrypt(text, left, right, &"1".repeat(text.len()));
                println!("plaintext right = {}", plaintext_assuming_right_hand);
                let common: String = plaintext_assuming_left_hand.chars().zip(plaintext_assuming_right_hand.chars()).map(|(c1, c2)| if c1 == '~' { c2 } else if c2 == '~' { c1 } else { '~' }).collect();
                println!("plaintext guaranteed = {}", common);
            } else if cipherkey.to_string().len() < text.len() {
                println!("Error: cipherkey lenght must be equal to plaintext length");
            } else {
                let plaintext = typing.decrypt(text, left, right, cipherkey);
                println!("{}", plaintext);
            }
            return;
        }
        _ => {
            println!("Error: Use encrypt or decrypt");
            return;
        }
    }

}
