use std::cmp::max;

fn main() {
    let s: String = String::from("babad");
    //let s: String = String::from("bb");
    //let s: String = String::from("cbbd");

    fn longest_palindrome_brute_force(s: &str) -> String {
        let c_to_s: Vec<char> = s.chars().collect();
        println!("vec_of_chars: {:?}", c_to_s);

        if s.len() <= 1 {
            return s.to_string();
        }

        let mut max_len: usize = 1;
        let mut max_str = c_to_s[0].to_string();

        for i in 0..c_to_s.len() {
            for j in i + 1..c_to_s.len() {
                //println!("{} > {} --> {}", j - i + 1, max_len, j - i + 1 > max_len);
                let a = c_to_s[i..j + 1].iter().collect::<String>();
                let a_rev = a.chars().rev().collect::<String>();

                //println!("A: {:?}", a);
                //println!("A rev: {:?}", a.chars().rev().collect::<String>());

                if j - i + 1 > max_len && a == a_rev {
                    max_len = j - i + 1;
                    max_str = a.clone();
                }
            }
        }

        max_str
    }

    println!("Longest palindrome substring: {:?}", longest_palindrome_brute_force(&s));
}
