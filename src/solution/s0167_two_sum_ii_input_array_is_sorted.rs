use std::cmp::{max, min};
use std::convert::TryInto;

/**
 * [167] Two Sum II - Input Array Is Sorted
 *
 * Given a 1-indexed array of integers numbers that is already sorted in non-decreasing order, find two numbers such that they add up to a specific target number. Let these two numbers be numbers[index1] and numbers[index2] where 1 <= index1 < index2 < numbers.length.
 * Return the indices of the two numbers, index1 and index2, added by one as an integer array [index1, index2] of length 2.
 * The tests are generated such that there is exactly one solution. You may not use the same element twice.
 * Your solution must use only constant extra space.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: numbers = [<u>2</u>,<u>7</u>,11,15], target = 9
 * Output: [1,2]
 * Explanation: The sum of 2 and 7 is 9. Therefore, index1 = 1, index2 = 2. We return [1, 2].
 * 
 * <strong class="example">Example 2:
 * 
 * Input: numbers = [<u>2</u>,3,<u>4</u>], target = 6
 * Output: [1,3]
 * Explanation: The sum of 2 and 4 is 6. Therefore index1 = 1, index2 = 3. We return [1, 3].
 * 
 * <strong class="example">Example 3:
 * 
 * Input: numbers = [<u>-1</u>,<u>0</u>], target = -1
 * Output: [1,2]
 * Explanation: The sum of -1 and 0 is -1. Therefore index1 = 1, index2 = 2. We return [1, 2].
 * 
 *  
 * Constraints:
 * 
 * 	2 <= numbers.length <= 3 * 10^4
 * 	-1000 <= numbers[i] <= 1000
 * 	numbers is sorted in non-decreasing order.
 * 	-1000 <= target <= 1000
 * 	The tests are generated such that there is exactly one solution.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/
// discuss: https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {

    fn binary_find_index(numbers: &Vec<i32>, target: i32) -> Option<usize> {
        let mut mindex = 0;
        let mut maxdex = numbers.len() - 1;
        while mindex <= maxdex {
            let potential_index = (maxdex + mindex) / 2;
            if numbers[potential_index] == target {
                return Some(potential_index);
            } else if numbers[potential_index] < target {
                mindex = potential_index + 1;
            } else {
                maxdex = potential_index - 1;
            }
        }
        return None;
    }

    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..numbers.len() {
            let n = numbers[i];
            let potential_match = Solution::binary_find_index(&numbers, target - n);
            match potential_match {
                Some (i2) if i != i2 => {
                    let res1: i32 = i.try_into().unwrap();
                    let res2: i32 = i2.try_into().unwrap();

                    return vec![min(res1,res2) + 1, max(res1,res2) + 1];
                },
                _ => continue,
            };
        }
        panic!("Oh no, no solution")
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input_list = vec![2,7,11,15];
        let target = 9;

        let res = Solution::two_sum(input_list, target);

        assert_eq!(vec![1,2], res);
    }

    #[test]
    fn test_2() {
        let input_list = vec![2,3,4];
        let target = 6;

        let res = Solution::two_sum(input_list, target);

        assert_eq!(vec![1,3], res);
    }

    #[test]
    fn test_3() {
        let input_list = vec![-1,0];
        let target = -1;

        let res = Solution::two_sum(input_list, target);

        assert_eq!(vec![1,2], res);
    }

    #[test]
    fn test_4() {
        let input_list = vec![1,2,3,4,4,9,56,90];
        let target = 8;

        let res = Solution::two_sum(input_list, target);

        assert_eq!(vec![4,5], res);
    }
}
