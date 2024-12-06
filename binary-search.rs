struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0_i32;
        let mut right = nums.len() as i32 -1;
        while left <= right {
            let mid = (left+right) / 2;
            if nums[mid as usize] < target {
                left = mid + 1;
            } else if nums[mid as usize] > target {
                right = mid - 1;
            } else {
                return mid;
            }
        }
        -1
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::search(vec![-1,0,3,5,9,12], 9), 4);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::search(vec![-1,0,3,5,9,12], 2), -1);
    }
}


