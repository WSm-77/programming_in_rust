use rand::seq::IndexedRandom;
use rand::rng;


fn generate_password(length: usize) -> String {
    let mut rng = rng();

    let lowercase: Vec<char> = ('a'..='z').collect();
    let uppercase: Vec<char> = lowercase.iter()
                                .map(|c: &char| c.to_ascii_uppercase())
                                .collect();
    let digits: Vec<char> = ('0'..='9').collect();
    let special_characters: Vec<char> = vec!['!', '@', '#', '$', '%', '^', '&', '*', '(', ')'];
    let allowed_characters: Vec<char> = [lowercase, uppercase, digits, special_characters].concat();

    let password: String = (0..length).into_iter()
        .map(|_| allowed_characters.choose(&mut rng).unwrap())
        .collect();

    return password;
}

fn main() {
    let pwd_len = 20;
    let password = generate_password(pwd_len);

    println!("{password}");
}
