const CONSONANTS: &str = "bcdfghjklmnpqrstvwxyz";

fn main() {
    println!("Input words: ");

    for word in get_input_data().trim().split_whitespace() {
        println!("{} => {}", word, transform_word(word));
    }
}

fn get_input_data() -> String {
    let mut data: String = String::new();
    std::io::stdin()
        .read_line(&mut data)
        .expect("Failed to read line!");

    data
}

fn transform_word(word: &str) -> String {
    let mut consonants = String::new();
    let mut result: String = String::new();
    let mut is_first_letter = true;

    for letter in word.chars() {
        if is_consonant(letter) {
            consonants.push(letter);
        } else {
            if is_first_letter {
                result.push_str(word);
                result.push_str("-hay");
            }
            break;
        }

        is_first_letter = false;
    }

    if consonants.len() > 0 {
        result.push_str(&word[consonants.len()..]);
        result.push_str("-");
        result.push_str(&consonants[..]);
        result.push_str("ay");
    }

    result
}

fn is_consonant(letter: char) -> bool {
    CONSONANTS.contains(letter)
}