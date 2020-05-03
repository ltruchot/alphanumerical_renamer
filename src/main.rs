use std::{fs, io};
use unicode_segmentation::UnicodeSegmentation;

fn main() -> std::io::Result<()> {
    let entries = fs::read_dir(".")?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    entries.iter().for_each(|e| {
        let name = e.file_name().unwrap().to_str().unwrap();
        let first_two = take_first_graphemes(name, 2);
        
        if take_last_grapheme(&first_two) == "_" {
            fs::rename(&name, ["0", name].join("")).unwrap_or_else(|err|{
                println!("Error during rename action: {:?}", err)
            });
        }
    });
    Ok(())
}

pub fn take_first_graphemes(s: &str, n: usize) -> String {
    s.graphemes(true).take(n).collect::<Vec<&str>>().join("")
}

pub fn take_last_grapheme(word: &str) -> String {
    let s = String::from(word);
    let last = s.graphemes(true).last();
    String::from(match last {
        Some(letter) => letter,
        None => "",
    })
}
