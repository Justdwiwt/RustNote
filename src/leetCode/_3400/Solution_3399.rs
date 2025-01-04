impl Solution {
    pub fn min_length(s: String, num_ops: i32) -> i32 {
        let s = s.as_bytes();
        let n = s.len();

        let is_valid = |mid: usize| -> bool {
            if mid == 1 {
                return (0..=1).any(|start_bit| {
                    let mut ops = num_ops;
                    let mut current_bit = b'0' + start_bit;
                    for &s_i in s {
                        if current_bit == s_i {
                            ops -= 1;
                        }
                        current_bit ^= 1;
                    }
                    ops >= 0
                });
            }

            let mut ops = num_ops;
            let mut count = 0;
            let mut last = b' ';
            for &s_i in s {
                if s_i == last {
                    count += 1;
                } else {
                    ops -= (count as i32) / (mid as i32 + 1);
                    last = s_i;
                    count = 1;
                }
            }
            ops -= (count as i32) / (mid as i32 + 1);
            ops >= 0
        };

        let (mut start, mut end) = (1, n);
        let mut ans = n;

        while start <= end {
            let mid = start + (end - start) / 2;
            if is_valid(mid) {
                ans = mid;
                end = mid - 1;
            } else {
                start = mid + 1;
            }
        }
        ans as i32
    }
}
