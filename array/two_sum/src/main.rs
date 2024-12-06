/*
O(n)
 */

use std::collections::HashMap;

fn main() {
    let nums: Vec<usize> = vec![2, 7, 11, 15];
    let target: usize = 9;

    fn two_sum(nums: Vec<usize>, target: usize) -> Vec<usize> {
        let mut map: HashMap<usize, usize> = HashMap::new();

        for i in 0..nums.len() as usize {
            map.insert(nums[i as usize], i);
        }

        for i in 0..nums.len() as usize {
            let c: usize = target - nums[i as usize];

            if map.contains_key(&c) && map.get(&c).is_some() {
                return vec![*map.get(&nums[i]).unwrap(), *map.get(&c).unwrap()];
            }
        }
        vec![]
    }

    println!("Output: {:?}", two_sum(nums, target));
}
