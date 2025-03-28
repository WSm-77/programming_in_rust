use std::io::{self, Write};

fn char_to_num(chr: char) -> u8 {
    return match chr {
        'X' => 10,
        _ => (chr as u8) - b'0'
    }
}

fn verify_isbn(isbn: &str) -> bool {
    let digits: Vec<u8> =  isbn.chars()
            .filter(|chr| chr.is_ascii_digit() || *chr == 'X')
            .map(char_to_num)
            .collect();

    if digits.len() != 10 {
        return false;
    }

    let mut result = 0;
    for i in (1..=10).rev() {
        let idx = 10 - i;

        // leave only remainders to prevent reslut variable overflow
        result = (result + (i as u8) * digits.get(idx).unwrap()) % 11;
    }

    return result == 0;
}

fn main() {
    print!("Enter ISBN-10 number to verify: ");
    io::stdout().flush().unwrap();

    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Failed to read ISBN-10");
    let user_input = user_input.trim();

    println!("{} is {} ISBN-10 number", user_input, if verify_isbn(&user_input) {"valid"} else {"invalid"});
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_valid_isbn() {
        assert!(verify_isbn("0000000000"));
        assert!(verify_isbn("99921-58-10-7"));
        assert!(verify_isbn("9971-5-0210-0"));
        assert!(verify_isbn("960-425-059-0"));
        assert!(verify_isbn("80-902734-1-6"));
        assert!(verify_isbn("0-9752298-0-X"));
        assert!(verify_isbn("0-8044-2957-X"));
    }

    #[test]
    fn test_invalid_isbn() {
        assert!( ! verify_isbn("99921-58-10-8"));
        assert!( ! verify_isbn("9971-5-0211-0"));
        assert!( ! verify_isbn("960-425-051-0"));
        assert!( ! verify_isbn("80-932734-1-6"));
        assert!( ! verify_isbn("0-9742298-0-X"));
        assert!( ! verify_isbn("0-8024-2957-X"));
        assert!( ! verify_isbn("some random input"));
    }
}
