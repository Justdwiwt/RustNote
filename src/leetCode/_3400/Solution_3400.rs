impl Solution {
    pub fn maximum_matching_indices(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        fn check(nums1: &Vec<i32>, nums2: &Vec<i32>) -> i32 {
            nums1
                .iter()
                .zip(nums2.iter())
                .filter(|(a, b)| a == b)
                .count() as i32
        }

        fn rotate_array(arr: &Vec<i32>, k: usize) -> Vec<i32> {
            let len = arr.len();
            let k = k % len;
            let mut rotated = Vec::with_capacity(len);
            rotated.extend_from_slice(&arr[len - k..]);
            rotated.extend_from_slice(&arr[..len - k]);
            rotated
        }

        let mut max_matching = 0;
        for i in 0..nums1.len() {
            let rotated_nums1 = rotate_array(&nums1, i);
            max_matching = max_matching.max(check(&rotated_nums1, &nums2));
        }

        max_matching
    }
}
