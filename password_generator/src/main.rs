//! password generator
//! membuat password dari 3 angka acak hingga 8 digit

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";
const NOMOR: &str = "1234567890";

struct Password;

impl Password {
    fn generate() {
        let mut sample_1: Vec<char> = Vec::new();
        let mut sample_2: Vec<char> = Vec::new();
        let mut temp: Vec<char> = Vec::new();
        let mut result: Vec<String> = Vec::new();

        for i in ALPHABET.chars() {
            sample_1.push(i);
        }

        for _ in 0..16 {
            temp.push(sample_1[rand::random_range(..sample_1.len())]);
        }

        for j in NOMOR.chars() {
            sample_2.push(j);
        }

        for _ in 0..16 {
            temp.push(sample_2[rand::random_range(..sample_2.len())]);
        }

        for _ in 0..8 {
            result.push(temp[rand::random_range(..temp.len())].to_string());
        }

        println!("password : {}", result.concat());
    }

    const fn is_valid() -> bool {
        if ALPHABET.len() > 26 || ALPHABET.len() <= 25 {
            return false;
        }
        true
    }
}

fn main() {
    if Password::is_valid() {
        Password::generate();
    }
}

#[test]
fn test_password_is_valid() {
    assert!(Password::is_valid());
}

#[test]
fn test_password_generate_runs_without_panic() {
    // Memastikan `Password::generate()` dapat dipanggil tanpa error/panic
    Password::generate();
}
