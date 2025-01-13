fn main() {
    let nums: Vec<i32> = vec![3, 2, 3, 2, 2];
    //let nums: Vec<i32> = vec![3, 2, 3];

    fn majority(nums: &Vec<i32>) -> i32 {
        let mut candidate: i32 = 0;
        let mut count = 0;

        for num in nums {
            if count == 0 {
                candidate = *num;
            }

            if num.eq(&candidate) {
                count += 1;
            }
            else {
                count -= 1;
            }
        }
        candidate
    }

    println!("{}", majority(&nums));
}
