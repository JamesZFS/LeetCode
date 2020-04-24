/*
搜索旋转排序数组
假设按照升序排序的数组在预先未知的某个点上进行了旋转。

( 例如，数组 [0,1,2,4,5,6,7] 可能变为 [4,5,6,7,0,1,2] )。

搜索一个给定的目标值，如果数组中存在这个目标值，则返回它的索引，否则返回 -1 。

你可以假设数组中不存在重复的元素。

你的算法时间复杂度必须是 O(log n) 级别。

示例 1:

输入: nums = [4,5,6,7,0,1,2], target = 0
输出: 4
示例 2:

输入: nums = [4,5,6,7,0,1,2], target = 3
输出: -1
*/

use crate::array::Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() { return -1; }
        if nums[0] == target { return 0; }
        let r = nums.len() - 1;
        if nums[r] == target { return r as i32; }
        // assert!(nums.first().unwrap() >= nums.last().unwrap());
        search(&nums, 0, r, target)
    }
}

/// Search in `a(l..r)`
fn search(a: &[i32], l: usize, r: usize, t: i32) -> i32 {
    if r - l <= 1 { return -1; } // base case
    let m = (l + r) >> 1;
    if a[m] == t { return m as i32; } // hit
    if a[l] < a[m] {
        if a[l] < t && t < a[m] { bin_search(a, l, m, t) } // O(log n)
        else { search(a, m, r, t) } // T(n/2)
    } else {
        if a[m] < t && t < a[r] { bin_search(a, m, r, t) } // O(log n)
        else { search(a, l, m, t) } // T(n/2)
    }
}

/// Binary search in `a(l..r)`
fn bin_search(a: &[i32], l: usize, r: usize, t: i32) -> i32 {
    if r - l <= 1 { return -1; } // base case
    let m = (l + r) >> 1;
    if a[m] == t { m as i32 } else if t < a[m] { bin_search(a, l, m, t) } else { bin_search(a, m, r, t) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let cases = vec![
            ((vec![4, 5, 6, 7, 0, 1, 2], 0), 4),
            ((vec![4, 5, 6, 7, 0, 1, 2], 5), 1),
            ((vec![4, 1], 0), -1),
            ((vec![4, 1], 1), 1),
            ((vec![4, 5, 6, 7, 0, 1, 2], 3), -1),
        ];
        for (input, output) in cases {
            assert_eq!(Solution::search(input.0, input.1), output);
        }
    }
}
