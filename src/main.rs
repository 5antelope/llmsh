pub mod gguf;

use inquire::Text;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{thread, time};

fn main() {
    let prompt = "> ";
    let exit = Arc::new(AtomicBool::new(false));

    while !exit.clone().load(Ordering::Relaxed) {
        let input = Text::new(&format!("{prompt} How can I help? ")).prompt();
        match input {
            Ok(input) => {
                let res = gguf::request(input);
                println!("{res}")
            }
            Err(e) => {
                eprintln!("An error ({e}) happened when asking for your name, try again later.");
                break;
            }
        }
        thread::sleep(time::Duration::from_millis(10));
    }
    println!("Bye");
}
