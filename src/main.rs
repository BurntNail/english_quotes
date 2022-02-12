mod chars;
mod theme;

use std::{collections::HashMap, path::Path, fmt::Display, fs::{OpenOptions, File}};
use std::io::Write;
use std::io::Result as IOResult;
use theme::Theme;
use chars::Character;

fn main() -> IOResult<()> {
    if std::env::var_os("RUST_BACKTRACE").is_none() {
        std::env::set_var("RUST_BACKTRACE", "full");
    }
    color_eyre::install().unwrap();

    let mut input = String::new();

    let mut chars: HashMap<Character, Vec<String>> = Default::default();
    let mut themes: HashMap<Theme, Vec<String>> = Default::default();

    println!("Enter a type, code and quote: ");

    loop {
        std::io::stdin().read_line(&mut input)?;

        let input_read = input.trim();

        if input_read == "exit" {
            break;
        }

        let space_pos = input_read.chars().position(|c| c == ' ').unwrap();
        let is_char = &input_read[..1] == "C";
        let code = &input_read[1..space_pos];
        let quote = &input_read[space_pos+1..];
        
        let list = if is_char {
            chars.entry(code.try_into().unwrap_or_default()).or_default()
        } else {
            themes.entry(code.try_into().unwrap_or_default()).or_default()
        };
        list.push(quote.to_string());
        
        input.clear();
    }

    print_hashmap("chars.md", chars)?;
    print_hashmap("themes.md", themes)?;

    Ok(())
}

fn print_hashmap (file_name: &str, map: HashMap<impl Display, Vec<String>>) -> IOResult<()>{
    let path = Path::new(file_name);

    let mut file = if File::open(path).is_ok() {
        OpenOptions::new().append(true).open(path)
    } else {
        File::create(path)
    }?;

    for (title, vec) in map {
        writeln!(file, "# {}", title)?;
        for st in vec {
            writeln!(file, " - *{}*", st)?;

        }
    }

    Ok(())
}