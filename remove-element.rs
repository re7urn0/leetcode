struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut cur = 0;
        for i in 0..nums.len() {
            if nums[i] != val {
                nums[cur] = nums[i];
                cur += 1;
            }
        }
        cur as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut v = vec![0,1,2,2,3,0,4,2];
        assert_eq!(Solution::remove_element(&mut v, 2), 5);
    }
}