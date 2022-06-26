use std::io;

fn main() {
    println!("Write the word to convert to pig latin below:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading input");
    println!("{}", pig_latin(String::from(input.trim())));
}

fn pig_latin(s: String) -> String {
    let rest_of_word = &s[1..];
    let first_letter = &s[0..1];
    let vowels = ["a", "e", "i", "o", "u"];
    if vowels.contains(&first_letter) {
        format!("{}-hay", s)
    } else {
        format!("{}-{}ay", rest_of_word, first_letter)
    }
}