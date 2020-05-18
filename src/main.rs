fn main() {
    let ciphertext =
        zilch::hex2bits("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");

    let plaintext = (0..=255)
        .map(|c| {
            let chars = zilch::str2bits(
                &std::iter::repeat(char::from(c))
                    .take(ciphertext.len())
                    .collect::<String>(),
            );
            zilch::bits2str(&zilch::xor(&ciphertext, &chars))
        })
        .map(|c| {
            println!("({}, {})", c, zilch::english_frequency_diff(&c));
            c
        })
        .collect::<Vec<_>>();
    // .min_by_key(|message| zilch::english_frequency_diff(&message))
    // .unwrap();

    // println!("{}", plaintext);
}
