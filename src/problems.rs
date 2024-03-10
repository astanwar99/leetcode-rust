pub struct Solution;

impl Solution {
    pub fn get_common(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut nc1 = 0;
        let mut nc2 = 0;

        loop {
            let ne1 = nums1.get(nc1).unwrap_or(&-1);
            let ne2 = nums2.get(nc2).unwrap_or(&-1);

            if ne1 == ne2 {
                return *ne1;
            } else if ne1 > ne2 {
                nc2 += 1;
            } else {
                nc1 += 1;
            }
        }
    }

    pub fn get_nearby_indexes(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() as i32 - 1;

        while left <= right {
            let mid = left + (right - left) / 2;
            let mid_val = nums.get(mid as usize).unwrap_or(&-1);
            let mid_prev = nums.get((mid - 1) as usize).unwrap_or(&-1);
            let mid_next = nums.get((mid + 1) as usize).unwrap_or(&-1);

            if (mid_prev <= &target && mid_next >= &target) || mid_val == &target {
                return mid - 1;
            } else if mid_val < &target && mid_next < &target {
                left = mid + 1;
            } else 

        -1
    }
}