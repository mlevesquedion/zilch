use std::fs;

fn brute_force(ciphertext: &str) -> String {
    (0..=255)
        .map(|c| {
            let chars = zilch::str2bits(
                &std::iter::repeat(char::from(c))
                    .take(ciphertext.len())
                    .collect::<String>(),
            );
            zilch::bits2str(&zilch::xor(&zilch::hex2bits(ciphertext), &chars))
        })
        .min_by_key(|message| zilch::english_frequency_diff(&message))
        .unwrap()
}

fn main() {
    let plaintext = fs::read_to_string("4.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let plain = brute_force(line);
            println!("{}", plain);
            plain
        })
        .min_by_key(|message| zilch::english_frequency_diff(&message))
        .unwrap();

    println!("{}", plaintext);
}
