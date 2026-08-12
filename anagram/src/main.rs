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
            count[(b - b'a') as usize] += 1;
        }

        for b in t.bytes() {
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
