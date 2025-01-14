fn main() {
    let mut s: [&str; 5] = ["h", "e", "l", "l", "o"];
    let mut i: usize = 0;
    let mut j: usize = s.len() - 1;

    while i<=j {
        let tmp = s[i];
        s[i] = s[j];
        s[j] = tmp;
        i += 1;
        j -= 1;
    }

    println!("{:?}", s);
}
