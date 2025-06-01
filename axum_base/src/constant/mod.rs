struct Solution;

// @lc code=begin

impl Solution {
    pub fn maximum_prime_difference(nums: Vec<i32>) -> i32 {
        let mut res = vec![];

        for (index,&value) in nums.iter().enumerate() {
            if Solution::is_prime(value) {
                res.push(index as i32);
            }
        }
        if res.len() < 2 {
            return 0;
        }
        res.sort_unstable();
        res[res.len() - 1] - res[0]
    }

    fn is_prime(num: i32) -> bool {
        for tmp_num in 2..num {
            if num % tmp_num == 0 {
                return false;
            }
        }
        true
    }
}


#[cfg(test)]
mod test{
    #[tokio::test]
    async fn test_maximum_prime_difference() {
        let nums = vec![1,7];
        assert_eq!(crate::constant::Solution::maximum_prime_difference(nums), 0);
    }
}