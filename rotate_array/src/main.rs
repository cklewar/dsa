fn rotate(mut nums: Vec<i32>, steps: i32) -> Vec<i32> {
    let k = steps % nums.len() as i32;

    if k == 0 {
        return nums;
    }

    for k in 0..steps {
        let mut right = nums.len() - 1;
        let mut right1 = right - 1;

        println!("rotate step: {}", k);

        while right >= 1 {
            println!("{} swap {}", nums[right], nums[right1]);
            let tmp = nums[right1];
            nums[right1] = nums[right];
            nums[right] = tmp;
            if right1 != 0 {
                right1 -= 1;
            }
            right -= 1;
        }
    }

    nums
}

fn main() {
    let nums: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7];
    let steps: i32 = 3;

    println!("{:?}", rotate(nums, steps));
}
