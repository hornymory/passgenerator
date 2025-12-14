use cli_clipboard::{ClipboardContext, ClipboardProvider};
use rand::Rng;
use std::env;
use std::thread::sleep;
use std::time::Duration;

struct Alphabet {
    chars: Vec<char>,
    len: usize,
}
impl Alphabet {
    fn new() -> Self {
        let chars = vec![
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
            'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '1', '2', '3', '4', '5', '6', '7', '8',
            '9', '0', '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '-', '_', '+', '=', '{',
            '}', '[', ']', '|', '\\', ';', ':', '\'', '\"', '<', '>', '?', '/', '.', ',', '~', '`',
            'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q',
            'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
        ];
        let len = chars.len();
        Self { chars, len }
    }
    fn get_random_symbol(&self) -> char {
        let mut rng = rand::thread_rng();
        self.chars[rng.gen_range(0..self.len)]
    }
}

struct Password {
    pass: String,
}
impl Password {
    fn new(alphabet: Alphabet, len: usize) -> Self {
        let mut pass = String::new();
        for _ in 0..len {
            pass.push(alphabet.get_random_symbol());
        }
        Self { pass}
    }
    fn get_password(&self) -> &str {
        &self.pass
    }

}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("you need enter length");
        return;
    }

    let len: usize = args[1].parse().expect("failed to parse len");

    if len < 8 {
        println!("password too short");
        return;
    } else {
        let pass = Password::new(Alphabet::new(), len);
        let mut ctx = ClipboardContext::new().unwrap();
        match ctx.set_contents(pass.get_password().to_string()) {
            Ok(_) => {
                sleep(Duration::from_millis(500));
            }
            Err(e) => {
                eprintln!("error to copy password to clipboard: {}", e);
            }
        }
        println!("{}", pass.get_password());
    }
}
