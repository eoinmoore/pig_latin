use std::io;

fn main() {
    let mut sentence = String::new();
    loop {
        println!("Please insert a sentence:");
        io::stdin()
            .read_line(&mut sentence)
            .expect("Failed to read line");
        break
    }
    sentence = sentence.trim().to_string();

    let mut pig_sentence = String::new();
    for word in sentence.split_whitespace() {
        pig_sentence.push_str(&piggify(word));
        pig_sentence.push(' ');
    };

    println!("{} -> {}", sentence, pig_sentence);
}

fn piggify(word: &str) -> String {
    assert!(word.len() > 0);
    let first_letter = word.chars().next().unwrap();
    let starts_with_vowel = match first_letter {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false,
    };

    let suffix: String;
    let root: &str;
    if starts_with_vowel {
        suffix = "-hay".to_string();
        root = word;
    } else {
        suffix = format!("-{}ay", first_letter);
        root = match word.len() {
            1 => word,
            _ => &word[1..],
        };
    }

    // let suffix = match first_letter {
    //     'a' | 'e' | 'i' | 'o' | 'u' => "-hay".to_string(),
    //     _ => format!("-{}ay", first_letter),
    // };
    // let root = match word.len() {
    //     1 => word,
    //     _ => &word[1..],
    // };
    format!("{}{}", root, suffix)
}
