// normalized freqs, from A to Z
const ENGLISH_FREQUENCIES: [f32; 26] = [
    0.084966, 0.02072, 0.045388, 0.033844, 0.1116, 0.01812, 0.0247, 0.03003, 0.075448, 0.00196,
    0.01101, 0.054893, 0.0301, 0.066544, 0.071635, 0.031671, 0.0019, 0.075809, 0.057351, 0.069509,
    0.036308, 0.01007, 0.01289, 0.0029, 0.01777, 0.00272,
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

pub fn english_frequency_diff(text: &str) -> u8 {
    (ENGLISH_FREQUENCIES
        .iter()
        .zip(letter_frequencies(text).iter())
        .map(|(f1, f2)| (f1 - f2).abs())
        .sum::<f32>()
        * 100.0) as u8
}
