fn main() {
    let ciphertext =
        zilch::hex2bits("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");

    for c in 0..=255 {
        let c = char::from(c);
        let chars = zilch::str2bits(
            &std::iter::repeat(c)
                .take(ciphertext.len())
                .collect::<String>(),
        );
        println!("{}", zilch::bits2str(&zilch::xor(&ciphertext, &chars)));
    }
}
