/*
三数之和
给你一个包含 n 个整数的数组 nums，判断 nums 中是否存在三个元素 a，b，c ，使得 a + b + c = 0 ？请你找出所有满足条件且不重复的三元组。

注意：答案中不可以包含重复的三元组。

示例：

给定数组 nums = [-1, 0, 1, 2, -1, -4]，
给定数组 nums = [-4, -1, -1, 0, 1, 2]，

满足要求的三元组集合为：
[
[-1, 0, 1],
[-1, -1, 2]
]
*/

use crate::array::Solution;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let n = nums.len();
        if n < 3 { return Vec::new(); }
        nums.sort_unstable();
        let mut res = Vec::new();
        for i in 0..n - 2 {
            let a = nums[i];
            if i >= 1 && a == nums[i - 1] { continue; } // a > a_prev
            let (mut l, mut r) = (i + 1, n - 1);
            while l < r {
                let (b, c) = (nums[l], nums[r]);
                let sum = a + b + c;
                if sum == 0 {  // Goal
                    res.push(vec![a, b, c]);
                    l += 1;
                    r -= 1;
                    while nums[l] == nums[l - 1] && l < r { l += 1; }
                    while nums[r] == nums[r + 1] && l < r { r -= 1; }
                } else if sum < 0 {
                    l += 1;
                } else {
                    r -= 1;
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let cases = vec![
            (vec![2, 2, 3, 3, 4, -4, -2, -2, -2, 0, 1, 2, 4, 6, 6], vec![[-4, -2, 6], [-4, 0, 4], [-4, 1, 3], [-4, 2, 2], [-2, -2, 4], [-2, 0, 2]]),
            (vec![], vec![]),
            (vec![1, 2, -3], vec![[-3, 1, 2]]),
            (vec![-1, 0, 1, 2, -1, -4], vec![[-1, -1, 2], [-1, 0, 1]]),
        ];
        for (input, expected) in cases {
            assert_eq!(Solution::three_sum(input), expected)
        }
    }
}
