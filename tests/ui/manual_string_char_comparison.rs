#![warn(clippy::manual_string_char_comparison)]
#![allow(unused)]

fn main() {
    let sentence = "Hello, world!";
    sentence.trim_end_matches(|c: char| c == '.' || c == '!' || c == '?');
    sentence.split(|c: char| c == '\n' || c == 'X');
    sentence.split(|c: char| matches!(c, '\n' | 'X'));
}
