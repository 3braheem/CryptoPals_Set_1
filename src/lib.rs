pub mod prob_1 {
    use hex;
    use base64;

    pub fn hex_to_b64(hex: &str) -> String {
        let bits = hex::decode(hex).unwrap();
        let b64 = base64::encode(bits);
        b64
    }
}

pub mod prob_2 {
    use hex;

    pub fn xor_buffers(hex1: &str, hex2: &str) -> String {
        let dec1 = hex::decode(hex1).unwrap();
        let dec2 = hex::decode(hex2).unwrap();
        let xor: Vec<u8> = dec1
            .into_iter()
            .zip(dec2.into_iter())
            .map(|(x, y)| x ^ y)
            .collect();

        hex::encode(xor)
    }
}

pub mod prob_3 {
    pub fn xor(bytes: &[u8]) -> String {
        let mut new: Vec<u8> = Vec::new();
        for byte in bytes {
            // I tried different chars based on rate of occurence
            // Once I got to 'r', everything became letters
            // Then went from 'r' all the way down.
            new.push(byte ^ 'x' as u8);
        }
        let a = std::str::from_utf8(&new).unwrap();
        a.to_owned()
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn prob_1() {
        // Problem #1 - hex -> b64
        use crate::prob_1::hex_to_b64;
        const HEX: &str  = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        const BASE_64: &str = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

        let b64 = hex_to_b64(HEX);
        assert_eq!(b64, BASE_64);
    }

    #[test]
    fn prob_2() {
        use crate::prob_2::xor_buffers;
        const BUF_1: &str = "1c0111001f010100061a024b53535009181c";
        const BUF_2: &str = "686974207468652062756c6c277320657965";
        const ANS: &str = "746865206b696420646f6e277420706c6179";

        let res = xor_buffers(BUF_1, BUF_2);
        assert_eq!(res, ANS);
    }

    #[test]
    fn prob_3() {
        use hex;
        use crate::prob_3::xor;
        const CIPHER_TEXT: &str = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
        const ANS: &str = "cOOKINGmcSLIKEAPOUNDOFBACON";

        let bytes = hex::decode(CIPHER_TEXT).unwrap();
        let ans = xor(&bytes);
        let ans = ans.replace("\u{0}", "");
        let ans = ans.replace("\u{7}", "");
        assert_eq!(&ans, ANS); 
    }
}
