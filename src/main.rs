use leetcode::problems;

fn main() {
    let nums1 = vec![1,2,3,6];
    let nums2 = vec![2,3,4,5];
    let s1 = problems::Solution::get_common(nums1, nums2);
    println!("Hello, world! {}", s1);
}