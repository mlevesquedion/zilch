const base64_table: [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '+', '/',
];
const padding: char = '=';

fn hex2bits(hex: &str) -> String {
    hex.chars()
        .map(|c| match c.to_ascii_lowercase() {
            '0' => "0000",
            '1' => "0001",
            '2' => "0010",
            '3' => "0011",
            '4' => "0100",
            '5' => "0101",
            '6' => "0110",
            '7' => "0111",
            '8' => "1000",
            '9' => "1001",
            'a' => "1010",
            'b' => "1011",
            'c' => "1100",
            'd' => "1101",
            'e' => "1110",
            'f' => "1111",
            _ => panic!("invalid hex: {}", c),
        })
        .collect::<Vec<_>>()
        .join("")
}

#[test]
fn test_hex2bits() {
    assert_eq!(
        &hex2bits("0123456789abcdef"),
        "0000000100100011010001010110011110001001101010111100110111101111"
    );
}

fn bits2u8(bits: &[char]) -> u8 {
    assert!(bits.len() <= 8, format!("too many bits: {}", bits.len()));
    let mut value: u8 = 0;
    for b in bits {
        value = value * 2
            + match b {
                '0' => 0,
                '1' => 1,
                _ => panic!("invalid bit: {}", b),
            }
    }
    value
}

#[test]
fn test_bits2u8() {
    assert_eq!(bits2u8(&vec!['1', '0', '0', '1', '0', '1']), 37);
}

fn hex2b64(hex: &str) -> String {
    hex2bits(hex)
        .chars()
        .collect::<Vec<char>>()
        .chunks(6)
        .map(|chunk| base64_table.get(bits2u8(chunk) as usize).expect("wtf"))
        .collect()
}

#[test]
fn test_hex2b64() {
    let hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let b64 = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
    assert_eq!(hex2b64(hex), b64);
}
