fn main() {
    //let s: String = String::from("abcabcbb");
    let s: String = String::from("bbb");
    //let s: String = String::from("pwwkew");

    fn substring(s: &str) -> (String, usize) {
        let mut substr: String = String::new();
        let mut candidate: String = String::new();

        for char in s.chars() {
            if !candidate.contains(&char.to_string()) {
                candidate.push(char);
            } else {
                if candidate.len() > substr.len() {
                    substr = candidate.clone();
                }
                candidate.clear();
                candidate.push(char);
            }
        }

        let l = substr.len();
        (substr, l)
    }

    let (substr, length) = substring(&s);
    println!("Substring: {:?}, Length: {}", substr, length);
}
