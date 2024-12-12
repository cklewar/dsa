use std::collections::HashMap;

fn main() {
    let strs:Vec<&str> = vec!("eat", "tea", "tan", "ate", "nat", "bat");
    let mut anagram_groups = HashMap::new();

    fn check(strs: &[&str], anagram_groups: &mut HashMap<String, Vec<String>>) {
        for s in strs {
            let mut chars: Vec<_> = s.chars().collect();
            chars.sort();
            let output: String = chars.into_iter().collect();

            if anagram_groups.contains_key(&output) {
                let values = anagram_groups.get_mut(&output).unwrap();
                values.push(s.to_string());
            } else {
                anagram_groups.insert(output, vec!(s.to_string()));
            }
        }

        println!("{:?}", anagram_groups.values());
    }

    check(&strs[..], &mut anagram_groups);
}
