use std::env;
use std::fs;
use std::io::{self, Read};

use arboard::Clipboard;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().skip(1).collect();

    let mut content = String::new();

    if args.is_empty() {
        // stdin
        io::stdin().read_to_string(&mut content)?;
    } else {
        for file in args {
            let file_content = fs::read_to_string(&file)?;
            content.push_str(&file_content);
        }
    }

    // вывод
    print!("{}", content);

    // копирование
    let mut clipboard = Clipboard::new()?;
    clipboard.set_text(content)?;

    Ok(())
}