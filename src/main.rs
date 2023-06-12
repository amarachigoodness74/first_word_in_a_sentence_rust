fn main() {
    let sentence = String::from("Hello world!");
    let sentence2 = String::from("Hello!");
    let sentence3 = String::from("Hi to the world!");
    let sentence4 = String::from("RustIsAwesome !!!");
    let get_first_word = first_word(&sentence);
    println!("First word from {} is {}", sentence, get_first_word);
    println!("First word from {} is {}", sentence2, first_word(&sentence2));
    println!("First word from {} is {}", sentence3, first_word(&sentence3));
    println!("First word from {} is {}", sentence4, first_word(&sentence4));
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}