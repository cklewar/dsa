use std::cmp;

fn main() {
    let nums: Vec<i32> = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
    let current_sum: i32 = 0;
    let max_sum: i32 = 0;

    fn max_subarray(nums: &Vec<i32>, mut current_sum: i32, mut max_sum: i32) -> i32 {
        for i in nums {
            current_sum = current_sum + i;
            max_sum = cmp::max(current_sum, max_sum);
            /*if current_sum > max_sum {
                max_sum = current_sum;
            }*/
            if current_sum < 0 {
                current_sum = 0;
            }
        }
        max_sum
    }
    
    println!("{}", max_subarray(&nums, current_sum, max_sum));

}
