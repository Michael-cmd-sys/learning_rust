/**
* Convert strings to Pig Latin. 
* The first consonant of each word is moved to the end of the word and ay is added, so first becomes irst-fay. 
* Words that start with a vowel have hay added to the end instead (apple becomes apple-hay). 
* Keep in mind the details about UTF-8 encoding!
**/
fn main() {
    let sample_text = String::from("We really are getting better by the number of daily practice we do in here. We are indeed moving forward and ahead.");
    
    let translated_text = translate_word(sample_text.clone());

    println!("The pig latin translation for \n{}\nis:\n{}", sample_text, translated_text);

}

fn translate_word(text: String) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let cleaned_text = text.replace(|c: char| c.is_ascii_punctuation(), "");

    cleaned_text 
        .trim()
        .to_lowercase()
        .split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            let first_char = chars.next().unwrap();

            if vowels.contains(&first_char) {
                format!("{}-hay", word)
            } else {
                format!("{}-{}ay", chars.as_str(), first_char)
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}
