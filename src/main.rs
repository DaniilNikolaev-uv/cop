use std::env;
use std::fs;
use std::io::{self, Read, Write};
use std::process::{Command, Stdio};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().skip(1).collect();

    let mut content = String::new();

    if args.is_empty() {
        io::stdin().read_to_string(&mut content)?;
    } else {
        for file in &args {
            content.push_str(&fs::read_to_string(file)?);
        }
    }

    print!("{}", content);
    copy_to_clipboard(&content)?;

    Ok(())
}

fn copy_to_clipboard(text: &str) -> Result<(), Box<dyn std::error::Error>> {
    let is_wayland = env::var("WAYLAND_DISPLAY").is_ok();

    let mut child = if is_wayland {
        Command::new("wl-copy")
            .stdin(Stdio::piped())
            .spawn()
            .map_err(|e| format!("wl-copy не найден: {e}. Установи wl-clipboard."))?
    } else {
        Command::new("xclip")
            .args(["-selection", "clipboard"])
            .stdin(Stdio::piped())
            .spawn()
            .map_err(|e| format!("xclip не найден: {e}. Установи xclip."))?
    };

    child.stdin.take().unwrap().write_all(text.as_bytes())?;

    let status = child.wait()?;
    if !status.success() {
        return Err(format!("Буфер обмена завершился с ошибкой: {status}").into());
    }

    Ok(())
}
