#![allow(dead_code)]

const BASE64_TABLE: [char; 65] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '+', '/', '=',
];
const BASE64_PADDING: u8 = 64;

pub fn hex2byte(hex: char) -> u8 {
    match hex {
        '0'..='9' => hex as u8 - ('0' as u8),
        'a'..='f' => hex as u8 - ('a' as u8) + 10,
        _ => panic!("invalid hex: {}", hex),
    }
}

pub fn hex2bytes(hex: &str) -> Vec<u8> {
    assert!(hex.len() % 2 == 0);

    hex.chars()
        .collect::<Vec<char>>()
        .chunks_exact(2)
        .map(|pair| (hex2byte(pair[0]) << 4) + hex2byte(pair[1]))
        .collect()
}

#[test]
fn test_hex2bytes() {
    assert_eq!(hex2bytes("42"), vec![0b01000010]);
    assert_eq!(hex2bytes("2af3"), vec![0b00101010, 0b11110011]);
}

fn lsb(n: u8) -> u8 {
    assert!(n <= 8);
    (1 << n) - 1
}

#[test]
fn test_lsb() {
    assert_eq!(lsb(1), 0b00000001);
    assert_eq!(lsb(2), 0b00000011);
    assert_eq!(lsb(3), 0b00000111);
    assert_eq!(lsb(4), 0b00001111);
    assert_eq!(lsb(5), 0b00011111);
    assert_eq!(lsb(6), 0b00111111);
    assert_eq!(lsb(7), 0b01111111);
}

fn msb(n: u8) -> u8 {
    assert!(n <= 7);
    !lsb(8 - n)
}

#[test]
fn test_msb() {
    assert_eq!(msb(1), 0b10000000);
    assert_eq!(msb(2), 0b11000000);
    assert_eq!(msb(3), 0b11100000);
    assert_eq!(msb(4), 0b11110000);
    assert_eq!(msb(5), 0b11111000);
    assert_eq!(msb(6), 0b11111100);
    assert_eq!(msb(7), 0b11111110);
}

pub fn hex2b64(hex: &str) -> String {
    hex2bytes(hex)
        .chunks(3)
        .flat_map(|chunk| {
            let mut indices: Vec<u8> = vec![0, 0];
            chunk.get(0).map(|a| {
                indices[0] += (a & msb(6)) >> 2;
                indices[1] += (a & lsb(2)) << 4;
            });
            chunk.get(1).map(|b| {
                indices.push(0);
                indices[1] += (b & msb(4)) >> 4;
                indices[2] += (b & lsb(4)) << 2;
            });
            chunk.get(2).map(|c| {
                indices.push(0);
                indices[2] += (c & msb(2)) >> 6;
                indices[3] += c & lsb(6);
            });
            indices.extend(std::iter::repeat(BASE64_PADDING).take(4 - indices.len()));
            indices
                .iter()
                .map(|i| BASE64_TABLE[*i as usize])
                .collect::<Vec<char>>()
        })
        .collect()
}

#[cfg(test)]
mod hex2b64_tests {
    use super::*;

    #[test]
    pub fn test_no_padding() {
        let hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let b64 = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
        assert_eq!(hex2b64(hex), b64);
    }

    #[test]
    pub fn test_padding() {
        let hex = "42";
        let b64 = "Qg==";
        assert_eq!(hex2b64(hex), b64);
    }
}

pub fn nibble2hex(nibble: u8) -> char {
    match nibble {
        0..=9 => char::from(nibble + ('0' as u8)),
        10..=15 => char::from(nibble - 10 + ('a' as u8)),
        _ => panic!("nibble cannot be converted to hex: {}", nibble),
    }
}

pub fn bytes2hex(bytes: Vec<u8>) -> String {
    bytes
        .iter()
        .flat_map(|b| vec![nibble2hex((b & msb(4)) >> 4), nibble2hex(b & lsb(4))])
        .collect()
}

#[test]
fn test_bytes2hex() {
    assert_eq!(bytes2hex(vec![0b01000010]), "42");
    assert_eq!(bytes2hex(vec![0b00101010, 0b11110011]), "2af3");
}

pub fn xor(bytes1: Vec<u8>, bytes2: Vec<u8>) -> Vec<u8> {
    bytes1
        .iter()
        .zip(bytes2.iter())
        .map(|(a, b)| a ^ b)
        .collect()
}

#[test]
pub fn test_xor() {
    let hex1 = "1c0111001f010100061a024b53535009181c";
    let hex2 = "686974207468652062756c6c277320657965";
    let expected = "746865206b696420646f6e277420706c6179";
    assert_eq!(bytes2hex(xor(hex2bytes(hex1), hex2bytes(hex2))), expected);
}

// TODO: str2bytes, bytes2str

// pub fn str2bytes(string: &str) -> Vec<u8> {
//     Vec::new()
// }

// #[test]
// fn test_str2bytes() {
//     let string = "hello world";
//     assert_eq!(str2bits(&string), expected);
// }

// pub fn bytes2str(bytes: &[u8]) -> String {
//     String::new()
// }

// #[test]
// fn test_bytes2str() {
//     let expected = "hello world";
//     assert_eq!(bits2str(&bits), expected);
// }
