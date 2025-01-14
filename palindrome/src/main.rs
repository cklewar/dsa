fn main() {
    //let s:&str = "A man, a plan, a canal: Panama";
    let s:&str =  "A woman, a plan, c canal: Panama";

    fn palindrome(s: &str) -> bool {
        let mut tmp =  s.chars().filter(|c| !c.is_whitespace()).collect::<String>();
        tmp.retain(|c| !r#"(),".;:'"#.contains(c));
        tmp.retain(|c| !c.is_whitespace());
        let new = tmp.to_lowercase();
        new.eq(&new.chars().rev().collect::<String>())
    }

    println!("Is palindrome: {}", palindrome(s));
}
