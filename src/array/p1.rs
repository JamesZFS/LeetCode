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
use std::collections::HashMap;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let n = nums.len();
        if n < 3 { return Vec::new(); }
        nums.sort_unstable();
        // println!("{:?}", nums);
        let mut res = Vec::new();
        let mut a_prev = NULL;
        for i in 0..n - 2 {
            let a = nums[i];
            if a == a_prev { continue; }
            let mut b_prev = NULL;
            for j in i + 1..n - 1 {
                let b = nums[j];
                if b == b_prev { continue; }
                let mut c_prev = NULL;
                for k in j + 1..n {
                    let c = nums[k];
                    if c == c_prev { continue; }
                    let sum = a + b + c;
                    if sum == 0 {
                        let new = vec![a, b, c];
                        res.push(new);
                    } else if sum > 0 { break; }
                    c_prev = c;
                }
                b_prev = b;
            }
            a_prev = a;
        }
        res
    }
}

const NULL: i32 = -2147483648 + 13;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let cases = vec![
            (vec![], vec![]),
            (vec![1, 2, -3], vec![[-3, 1, 2]]),
            (vec![-1, 0, 1, 2, -1, -4], vec![[-1, -1, 2], [-1, 0, 1]]),
            (vec![2, 2, 3, 3, 4, -4, -2, -2, -2, 0, 1, 2, 4, 6, 6], vec![[-4, -2, 6], [-4, 0, 4], [-4, 1, 3], [-4, 2, 2], [-2, -2, 4], [-2, 0, 2]])
        ];
        for (input, expected) in cases {
            assert_eq!(Solution::three_sum(input), expected)
        }
    }
}
