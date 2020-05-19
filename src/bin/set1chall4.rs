use std::fs;

fn brute_force(ciphertext: &str) -> String {
    let ciphertext = zilch::hex2bytes(ciphertext);
    (0..=255)
        .map(|c| {
            let chars = zilch::str2bytes(
                &std::iter::repeat(char::from(c))
                    .take(ciphertext.len())
                    .collect::<String>(),
            );
            zilch::bytes2str(&zilch::xor(&ciphertext, &chars))
        })
        .max_by_key(|message| zilch::english_score(&message))
        .unwrap()
}

fn main() {
    let plaintext = fs::read_to_string("4.txt")
        .unwrap()
        .lines()
        .map(brute_force)
        .max_by_key(|message| zilch::english_score(&message))
        .unwrap();

    println!("{}", plaintext);
}
