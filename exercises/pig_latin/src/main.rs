fn main() {
    let mut sentence1 = String::from("I am speaking pig latin");
    let mut sentence2 = String::from("An apple a day keeps the doctor away");

    to_pig_latin(&mut sentence1);
    to_pig_latin(&mut sentence2);

    println!("sentence 1: {sentence1}");
    println!("sentence 2: {sentence2}");
}

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

fn to_pig_latin(sentence: &mut String) {
    let split_sentence: Vec<&str> = sentence.split(" ").collect();

    let mut new_sentence: Vec<String> = Vec::new();

    for i in split_sentence.iter() {
        let mut chars = i.chars();

        if let Some(first_letter) = chars.next() {
            if VOWELS.contains(&first_letter.to_ascii_lowercase()) {
                new_sentence.push(format!("{i}-hay"))
            } else {
                let rest_of_word: String = chars.collect();
                new_sentence.push(format!("{}-{}ay", rest_of_word, first_letter));
            }
        }
    }

    *sentence = new_sentence.join(" ");
}
