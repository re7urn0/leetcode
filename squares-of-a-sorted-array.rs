struct Solution;

impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![0; nums.len()];
        let mut left = 0;
        let mut right = nums.len() - 1;

        let mut cur = nums.len();
        while left <= right {
            cur -= 1;
            if nums[left] * nums[left] >= nums[right] * nums[right] {
                res[cur] = nums[left] * nums[left];
                left += 1;
            } else {
                res[cur] = nums[right] * nums[right];
                right -= 1;
            }

        }
        res 
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::sorted_squares(vec![-7,-3,2,3,11]), [4,9,9,49,121]);
    }
}

