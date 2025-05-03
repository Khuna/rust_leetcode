use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, usize> = HashMap::with_capacity(nums.len());
    for (i, num) in nums.iter().enumerate() {
        match map.get(&num) {
            Some(&j) => return vec![i as i32, j as i32],
            None => {
                map.insert(target - num, i);
            }
        }
    }
    unreachable!();
}