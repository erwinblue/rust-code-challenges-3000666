mod run_length_encoding {
    pub fn encode(text: &str) -> String {
        //todo!()
        let mut rle_encoded = String::new();
        let mut string_chars= text.chars().peekable();
        
        while let Some(c) = string_chars.next() {
            let mut repeat_count = 1u32;
            while string_chars.peek() == Some(&c) {
                string_chars.next();
                repeat_count += 1;
                if repeat_count == 9 {
                    break;
                }
            }
            rle_encoded.push_str(&repeat_count.to_string());
            rle_encoded.push(c);
        }
        rle_encoded
    }
    
    pub fn decode(text: &str) -> String {
        //todo!()
        let mut rle_decoded = String::new();
        let mut count = String::new();
        for c in text.chars().into_iter() {
            if c.is_digit(10) {
                count.push_str(&c.to_string());
            } else if c.is_alphabetic() {
                let num = match count.parse::<usize>() {
                    Ok(n) => n,
                    Err(_) => panic!("ERROR: Cannot determine repeat count!")
                };
                let p = c.to_string().repeat(num);
                rle_decoded.push_str(&p);
                count.clear();
            }
        }
        rle_decoded
    }
}

fn main() {
    use run_length_encoding::*;
    dbg!(encode("aabc"));
    dbg!(decode("2a1b1c"));
}

#[test]
fn abc() {
    use run_length_encoding::*;

    assert_eq!(encode("abc"), "1a1b1c");
}

#[test]
fn round_trip() {
    use run_length_encoding::*;

    let input = "LinkedIn";
    println!("{}", encode(input));
    assert_eq!(decode(&encode(input)), input);
}

#[test]
fn long_run() {
    use run_length_encoding::*;

    let input = "AAAAA AAAAAAAAAA AAAAAAAAAAAAAAAAAAAA";
    assert_eq!(encode(input), "5A1 9A1A1 9A9A2A");
}
