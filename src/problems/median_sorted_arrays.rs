pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let full_len = nums1.len() + nums2.len();
    let mut i1 = 0;
    let mut i2 = 0;
    let mut prev_value = 0;
    let mut curr_value = 0;
    match (nums1.len(), nums2.len()) {
        (0, 0) => return 0.0,
        (0, 1) => return f64::from(nums2[0]),
        (1, 0) => return f64::from(nums1[0]),
        (1, 1) => return f64::from(nums1[0] + nums2[0]) / 2.0,
        (0, _) => { 
            match nums2.len() % 2 == 1 {
                true => return f64::from(nums2[nums2.len() / 2]),
                false => return f64::from(nums2[nums2.len() / 2] + nums2[nums2.len() / 2 - 1]) / 2.0
            }
        },
        (_, 0) => { 
            match nums1.len() % 2 == 1 {
                true => return f64::from(nums1[nums1.len() / 2]),
                false => return f64::from(nums1[nums1.len() / 2] + nums1[nums1.len() / 2 - 1]) / 2.0
            }
        },
            
        _ => ()
    }
    while (i1 + i2) <=  full_len / 2 {
        if i1 == nums1.len() {
            prev_value = curr_value;
            curr_value = nums2[i2];
            i2 += 1;

        } else if i2 == nums2.len() {
            prev_value = curr_value;
            curr_value = nums1[i1];
            i1 += 1;
        }
        else if nums1[i1] <= nums2[i2] {
            prev_value = curr_value;
            curr_value = nums1[i1];
            i1 += 1;
        } else {
            prev_value = curr_value;
            curr_value = nums2[i2];
            i2 += 1;
        }

    }

    if full_len % 2 == 0 {
        f64::from(prev_value + curr_value) / 2.0
    } else {
        f64::from(curr_value)
    }
}


