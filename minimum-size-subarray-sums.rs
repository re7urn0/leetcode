struct Solution;

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut res = (nums.len() + 1) as i32;
        let mut sum = 0;

        let mut i = 0;
        for (j, val) in nums.iter().enumerate() {
            sum += val;
            while sum >= target {
                let cur_len = (j - i + 1) as i32;
                if cur_len < res {
                    res = cur_len;
                }
                sum -= nums[i];
                i += 1;
            }
        }

        if res > nums.len() as i32 { 0 } else { res }
    }
}