use std::collections::HashSet;

fn main() {
    //let s:&str = "A man, a plan, a canal: Panama";
    let s: &str = "A woman, a plan, c canal: Panama";

    fn palindrome(s: &str) -> bool {
        if s.len() == 0 {
            return true;
        }

        let mut set: HashSet<char> = HashSet::new();
        let mut new = String::new();

        for i in 0..26 {
            let c = char::from('a' as u8 + i);
            set.insert(c);
        }

        for i in 0..26 {
            let c = char::from('A' as u8 + i);
            set.insert(c);
        }

        for i in s.chars() {
            if set.contains(&i) {
                new.push(i.to_lowercase().next().unwrap());
            }
        }

        //Using builtin func filter, retain and to_lowercase
        //let mut tmp =  s.chars().filter(|c| !c.is_whitespace()).collect::<String>();
        //tmp.retain(|c| !r#"(),".;:'"#.contains(c));
        //tmp.retain(|c| !c.is_whitespace());
        //let new = tmp.to_lowercase();
        //new.eq(&new.chars().rev().collect::<String>())

        let n = new.len();

        for (i, _) in new.char_indices() {
            if new.as_bytes()[i] != new.as_bytes()[n - 1 - i] {
                return false;
            }
        }

        true
    }

    println!("Is palindrome: {}", palindrome(s));
}
