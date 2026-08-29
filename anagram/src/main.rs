// anagram
// jika s = "anagram" dan t = "nagaram" maka return true
// syarat s == t

struct Solution;

impl Solution {
    fn is_anagram(s: &str, t: &str) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut count = [0i32; 26];

        for b in s.bytes() {
            if !b.is_ascii_lowercase() {
                return false;
            }
            count[(b - b'a') as usize] += 1;
        }

        for b in t.bytes() {
            if !b.is_ascii_lowercase() {
                return false;
            }
            count[(b - b'a') as usize] -= 1;
        }

        count.iter().all(|n| *n == 0)
    }
}

fn main() {
    let a = String::from("kasur");
    let a2 = String::from("rusak");

    let result = Solution::is_anagram(&a, &a2);

    println!("{a} dan {a2} -> {result}");
}

#[test]
fn test_is_anagram_valid() {
    assert!(Solution::is_anagram("anagram", "nagaram"));
    assert!(Solution::is_anagram("kasur", "rusak"));
}

#[test]
fn test_is_anagram_invalid() {
    assert!(!Solution::is_anagram("rat", "car"));
    assert!(!Solution::is_anagram("hello", "world"));
}

#[test]
fn test_is_anagram_bedalength() {
    assert!(!Solution::is_anagram("a", "ab"));
    assert!(!Solution::is_anagram("ab", "a"));
}

#[test]
fn test_is_anagram_case_sensitive() {
    // Implementasi saat ini case-sensitive
    assert!(!Solution::is_anagram("Anagram", "nagaram"));
}
