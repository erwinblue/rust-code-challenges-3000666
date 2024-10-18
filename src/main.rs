use std::fmt::Display;
use std::str::FromStr;


// part of instructors elegant solution
#[derive(Debug)]
enum ParseColorError {
    Invalid,
    RedOutOfBounds,
    GreenOutOfBounds,
    BlueOutOfBounds
}

#[derive(Debug)]
struct ParseRgbError {
    error: String
}


#[derive(Debug, PartialEq)]
//struct Rgb; // TODO: design data structure
struct Rgb {
    raw: String,
    red: u8,
    green: u8,
    blue: u8
}

trait RgbChannels {
    fn r(&self) -> u8;

    fn g(&self) -> u8;

    fn b(&self) -> u8;
}

impl RgbChannels for Rgb {
    // TODO: implement trait
    fn r(&self) -> u8 {
        self.red
    }

    fn g(&self) -> u8 {
        self.green
    }

    fn b(&self) -> u8 {
        self.blue
    }
}

impl FromStr for Rgb {
    // TODO: implement trait
    /*
    type Err = ParseRgbError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // initialize Rgb struct. input string will be something like "#ab12ef"
        let mut rgb: Rgb = Rgb { raw: s.to_owned().to_lowercase(), red: 0, green: 0, blue: 0};
        // break down the string into individual characters
        let mut raw_chars: Vec<char> = s.chars().into_iter().collect();
        // remove the '#' symbol at beginning of string, boolean has_hash is needed
        // to error out later after making some sanity checks
        let mut has_hash: bool = false;
        if s.starts_with("#") {
            raw_chars.remove(0);
            has_hash = true;
        }
        // make sure we have enough number of digits
        if raw_chars.len() < 6 {
            return Err(ParseRgbError { error: "Input hex color string is too short!".to_string() });
        } else if raw_chars.len() > 6 {
            return Err(ParseRgbError { error: "Input hex color string is too long!".to_string() });
        }
        // check to make sure each character is for a hex digit
        for c in raw_chars.iter() {
            if !c.is_ascii_hexdigit() {
                return Err(ParseRgbError { error: "Input hex color string contains invalid characters!".to_string() })
            }
        }
        // error if leading hash is missing
        if !has_hash {
            return Err(ParseRgbError { error: "Input hex color string is invalid!  Missing leading hash (#).".to_string() })
        }
        // create the r, g, b slices and convert to u8 integers as needed for our Rgb struct
        for (i, parts) in raw_chars.chunks(2).into_iter().enumerate() {
            let hex_code: String = parts.into_iter().collect();
            let hex = match u8::from_str_radix(&hex_code, 16) {
                Ok(h) => h,
                Err(_) => return Err(ParseRgbError { error: "Catastrophic Error converting string to hex!".to_string() })
            };
            match i {
                0 => { rgb.red = hex },
                1 => { rgb.green = hex },
                2 => { rgb.blue = hex },
                _ => continue
            }
        }
        Ok(rgb)
    }*/

    // Instructor's more elegant solution
    type Err = ParseColorError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(hex) = s.strip_prefix('#') {
            let r = u8::from_str_radix(&hex[0..2], 16)
                .or_else(|_| Err(ParseColorError::RedOutOfBounds))?;
            let g = u8::from_str_radix(&hex[2..4], 16)
                .or_else(|_| Err(ParseColorError::BlueOutOfBounds))?;
            let b = u8::from_str_radix(&hex[4..6], 16)
                .or_else(|_| Err(ParseColorError::GreenOutOfBounds))?;

            //Ok(Rgb { r, g, b })
            Ok(Rgb { raw: s.to_owned().to_lowercase(), red: r, green: g, blue: b })
        } else {
            // no leading #
            Err(ParseColorError::Invalid)
        }
    }
}

impl Display for Rgb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:02x}{:02x}{:02x}", self.r(), self.g(), self.b())
    }
}

fn main() {
    // 
}

#[test]
fn every_color() {
    let colors = (0_u8..255).zip(0_u8..255).zip(0_u8..255);

    for ((r, g), b) in colors {
        let hex = format!("#{:02x}{:02x}{:02x}", r, g, b);
        let color: Rgb = hex.parse().unwrap();
        assert_eq!(hex, format!("{}", color));
    }
}

#[test]
#[should_panic]
fn too_short () {
    let _: Rgb = "1234".parse().unwrap();
}

#[test]
#[should_panic]
fn not_a_hex_code () {
    let _: Rgb = "?".parse().unwrap();
}

#[test]
#[should_panic]
fn invalid_literals () {
    let _: Rgb = "?".parse().unwrap();
}

#[test]
#[should_panic]
fn no_leading_hash() {
    let _: Rgb = "aabbcc".parse().unwrap();
}

#[test]
#[should_panic]
fn out_of_bounds() {
    let _: Rgb = "00gg00".parse().unwrap();
}

