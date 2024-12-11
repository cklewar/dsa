use std::collections::HashMap;

fn main() {
    let map: HashMap<String, i32> = HashMap::from([
        ("I".to_string(), 1),
        ("V".to_string(), 5),
        ("X".to_string(), 10),
        ("L".to_string(), 50),
        ("C".to_string(), 100),
        ("D".to_string(), 500),
        ("M".to_string(), 1000),
    ]);

    fn roman_to_int(s: &str, map: &HashMap<String, i32>) -> i32 {
        let mut sum: i32 = 0;
        let chars: Vec<char> = s.chars().collect();

        println!("str: {:?} | Vec<char>: {:?}", s, chars);

        for i in 0..chars.len() {
            let curr = map.get(&chars[i].to_string()).unwrap();

            if i < chars.len() - 1 && curr < map.get(&chars[i + 1].to_string()).unwrap() {
                sum = sum - map[&chars[i].to_string()];
            } else {
                sum = sum + map[&chars[i].to_string()];
            }
        }

        sum
    }

    println!("SUM: {}", roman_to_int("MDCCCCLXXXIIII", &map));
}
