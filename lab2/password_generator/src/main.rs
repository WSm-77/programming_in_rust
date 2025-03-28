use rand::seq::IndexedRandom;
use rand::rng;
use std::io::{self, Write};

const LOWERCASE_OPTION: usize = 0;
const UPPERCASE_OPTION: usize = 1;
const DIGITS_OPTION: usize = 2;
const SPECIAL_OPTION: usize= 3;

fn generate_password(length: usize, charset: &[String]) -> String {
    let mut rng = rng();

    let lowercase: Vec<char> = ('a'..='z').collect();
    let uppercase: Vec<char> = lowercase.iter()
                                .map(|c: &char| c.to_ascii_uppercase())
                                .collect();
    let digits: Vec<char> = ('0'..='9').collect();
    let special_characters: Vec<char> = vec!['!', '@', '#', '$', '%', '^', '&', '*', '(', ')'];
    let mut allowed_characters: Vec<char> = vec![];
    let mut checked_option = [false; 4];

    for option in charset {
        if option == "lowercase" && !checked_option[LOWERCASE_OPTION] {
            allowed_characters = allowed_characters.into_iter().chain(lowercase.clone().into_iter()).collect();
            checked_option[LOWERCASE_OPTION] = true;
        }
        else if option == "uppercase" && !checked_option[UPPERCASE_OPTION] {
            checked_option[UPPERCASE_OPTION] = true;
            allowed_characters = allowed_characters.into_iter().chain(uppercase.clone().into_iter()).collect();
        }
        else if option == "digits" && !checked_option[DIGITS_OPTION] {
            allowed_characters = allowed_characters.into_iter().chain(digits.clone().into_iter()).collect();
            checked_option[DIGITS_OPTION] = true;
        }
        else if option == "special" && !checked_option[SPECIAL_OPTION]{
            allowed_characters = allowed_characters.into_iter().chain(special_characters.clone().into_iter()).collect();
            checked_option[SPECIAL_OPTION] = true;
        }
    }

    let password: String = (0..length).into_iter()
        .map(|_| allowed_characters.choose(&mut rng).unwrap())
        .collect();

    return password;
}

fn main() {
    print!("Enter password length: ");
    io::stdout().flush().unwrap();

    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Failed to read a password length");

    let pwd_len: usize = match user_input.trim().parse() {
        Ok(value) => value,
        Err(_) => {
            println!("Invalid input!!! Please, provide positive integer");
            return;
        }
    };

    print!("Enter password comma-separated options (available: lowercase, uppercase, digits, special): ");
    io::stdout().flush().unwrap();

    user_input.clear();
    io::stdin().read_line(&mut user_input).expect("Failed to read options");

    let options: Vec<String> =  user_input.split(",")
        .into_iter()
        .map(|option| option.trim().to_string())
        .collect();

    // let options = [String::from("uppercase"), String::from("lowercase"), String::from("special")];
    // let options = [String::from("digits"), String::from("special"), String::from("special")];
    let password = generate_password(pwd_len, options.as_slice());

    println!("{password}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_passwords_len() {
        for i in 1..=5 {
            let password_len = i * 5;
            let password = generate_password(password_len, &["lowercase".to_string()]);
            assert_eq!(password_len, password.len());
        }
    }
}
