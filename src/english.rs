// normalized freqs, from A to Z
const ENGLISH_FREQUENCIES: [f32; 26] = [
    8.4966, 2.072, 4.5388, 3.3844, 11.160, 1.812, 2.470, 3.003, 7.5448, 0.196, 1.101, 5.4893, 3.01,
    6.6544, 7.1635, 3.1671, 0.19, 7.5809, 5.7351, 6.9509, 3.6308, 1.007, 1.289, 0.290, 1.777,
    0.272,
];

fn is_alpha(c: char) -> bool {
    c.is_alphabetic() && c as u8 <= 122
}

fn letter_frequencies(text: &str) -> [f32; 26] {
    let mut frequencies = [0.0; 26];

    text.chars()
        .filter(|c| is_alpha(*c))
        .map(|c| c.to_ascii_uppercase() as u8 - ('A' as u8))
        .for_each(|i| frequencies[i as usize] += 1.0);

    let total = frequencies.iter().sum::<f32>();
    if total == 0.0 {
        return frequencies;
    }

    for i in 0..26 {
        frequencies[i] /= total;
    }

    frequencies
}

pub fn english_frequency_diff(text: &str) -> f32 {
    ENGLISH_FREQUENCIES
        .iter()
        .zip(letter_frequencies(text).iter())
        .map(|(f1, f2)| (f1 - f2).abs())
        .sum::<f32>()
}
