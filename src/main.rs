use core::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct ParseIsbnError {
    error: String
}

impl fmt::Display for ParseIsbnError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.error)
    }
}

#[derive(Debug, PartialEq)]
struct Isbn {
    raw: String,
    digits: Vec<u8>,
}

impl FromStr for Isbn {
    //type Err = (); // TODO: replace with appropriate type
    type Err = ParseIsbnError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        //todo!();        

        // Convert string to a vector of u8 digits
        /*let isbn13: Vec<u8> = s.replace("-","")
            .chars()
            .into_iter()
            .map(|x| {
                x.to_string().parse::<u8>()
                    .map_err(|_| ParseIsbnError { error: "Cannot parse number from ISBN".to_string() } )
                    .unwrap_or(0u8)
                })
            .collect();*/
        let mut parse_err: bool = false;
        let isbn13: Vec<u8> = s.replace("-", "").chars()
            .map(|x| {
                let y = match x {
                    '0'..='9' => match x.to_digit(10) {
                        Some(a) => a as u8,
                        None => return Err(ParseIsbnError { error: "Cannot parse ISBN number!".to_string() })
                    },
                    _ => return Err(ParseIsbnError { error: "Cannot parse ISBN number!".to_string() })
                };
                Ok(y)
            })
            // a lazy way to unwrap without using unwrap, or its variants...
            .map(|x| {
               let y = match x {
                   Ok(z) => z,
                   Err(_) => { parse_err = true; 0u8 }
               };
               y
            })
            .collect();

        if parse_err {
            return Err(ParseIsbnError { error: "Cannot parse ISBN number!".to_string() });
        } else if isbn13.len() > 13 {
            return Err(ParseIsbnError { error: "ISBN number is too long".to_string() });
        } else if isbn13.len() < 13 {
            return Err(ParseIsbnError { error: "ISBN number is too short".to_string() });
        }

        // Get the ISBN13 checksum number from provided ISBN
        let check_number = isbn13[12];

        // Compute the checksum number from the ISBN digits
        let computed_check = calculate_check_digit(&isbn13[0..=11]);

        // Check the computed and provided check sum
        if check_number != computed_check {
            return Err(ParseIsbnError { error: "Invalid ISBN number!".to_string() });
        } else {
            return Ok(Isbn {
                raw: s.to_string(),
                digits: isbn13
            })
        }
    }
}

impl std::fmt::Display for Isbn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.raw)
    }
}

// https://en.wikipedia.org/wiki/International_Standard_Book_Number#ISBN-13_check_digit_calculation
fn calculate_check_digit(digits: &[u8]) -> u8 {
    //todo!()
    let mut count = 1u8;
    let sum_of_12 = digits.iter().fold(0,|acc: u32, x| {
            let y = match  count % 2 {
                0 => x * 3,
                _ => x * 1
            };
            count += 1;
            acc + y as u32
        });
    let rem = 10 - (sum_of_12 % 10);

    // rem is assumed to be 0 to 10
    if rem == 10 {
        return 0u8;
    } else {
        return rem as u8;
    }
}

fn main() {
    let rust_in_action: Isbn = "978-3-16-148410-0".parse().unwrap();

    println!("Rust in Action's ISBN-13 ({})is valid!", rust_in_action);
}

#[test]
fn can_correctly_calculate_check_digits() {
    let cases = [
        ([9_u8, 7, 8, 1, 8, 6, 1, 9, 7, 8, 7, 6], 9_u8),
        ([9_u8, 7, 8, 3, 1, 6, 1, 4, 8, 4, 1, 0], 0_u8),
    ];

    for (case, check) in cases.iter() {
        let actual = calculate_check_digit(case);
        println!("{:?} -> {}?  {}", &case, check, actual);
        assert_eq!(calculate_check_digit(case), *check)
    }
}

#[test]
fn rust_in_action() {
    let _: Isbn = "978-3-16-148410-0".parse().unwrap();
}

#[test]
fn rust_invalid_isbn() {
    let invalid: Result<_, ParseIsbnError> = Err(ParseIsbnError { 
        error: "Cannot parse ISBN number!".to_string() });
    assert_eq!("978-3-16-14841C-0".parse::<Isbn>(), invalid);
}
