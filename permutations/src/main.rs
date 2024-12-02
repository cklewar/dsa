fn main() {
    fn backtrack(
        result: &mut Vec<Vec<i32>>,
        curr: &mut Vec<i32>,
        used: &mut Vec<bool>,
        nums: &Vec<i32>,
    ) {
        if curr.len() == nums.len() {
            result.push(curr.clone());
            return;
        }
        for n in 0..nums.len() {
            if used[n] {
                continue;
            }
            curr.push(nums[n]);
            used[n] = true;

            backtrack(result, curr, used, nums);
            curr.pop();
            used[n] = false;
        }
    }

    fn permutate(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![];
        let mut used: Vec<bool> = vec![false; nums.len()];
        let mut curr: Vec<i32> = vec![];

        backtrack(&mut result, &mut curr, &mut used, &nums);
        result
    }

    println!("{:?}", permutate(vec![1, 2, 3]));
}
