fn main() {
    let ciphertext =
        zilch::hex2bytes("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");

    let plaintext = (0..=255)
        .map(|c| {
            let chars = zilch::str2bytes(
                &std::iter::repeat(char::from(c))
                    .take(ciphertext.len())
                    .collect::<String>(),
            );
            zilch::bytes2str(&zilch::xor(&ciphertext, &chars))
        })
        .max_by_key(|message| zilch::english_score(&message))
        .unwrap();

    println!("{}", plaintext);
}
