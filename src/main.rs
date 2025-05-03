mod problems;

use crate::problems::median_sorted_arrays::find_median_sorted_arrays;

fn main() {
    let nums1 = vec![2, 3];
    let nums2 = vec![1];
    // 2 3 7 10 11 12 15 18
    let result = find_median_sorted_arrays(nums1, nums2);
    println!("{result:?}");
}