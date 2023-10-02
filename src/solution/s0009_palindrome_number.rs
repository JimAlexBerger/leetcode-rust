/**
 * [9] Palindrome Number
 *
 * Given an integer x, return true if x is a <span data-keyword="palindrome-integer">palindrome</span>, and false otherwise.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: x = 121
 * Output: true
 * Explanation: 121 reads as 121 from left to right and from right to left.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: x = -121
 * Output: false
 * Explanation: From left to right, it reads -121. From right to left, it becomes 121-. Therefore it is not a palindrome.
 * 
 * <strong class="example">Example 3:
 * 
 * Input: x = 10
 * Output: false
 * Explanation: Reads 01 from right to left. Therefore it is not a palindrome.
 * 
 *  
 * Constraints:
 * 
 * 	-2^31 <= x <= 2^31 - 1
 * 
 *  
 * Follow up: Could you solve it without converting the integer to a string?
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/palindrome-number/
// discuss: https://leetcode.com/problems/palindrome-number/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        fn make_palindrome(pre: i32, x: i32) -> i32 {            
            if (x < 10) {
                return pre * 10 + x;
            } else {
                let d = x % 10;
                return make_palindrome(pre * 10 + d, x / 10);
            }        
        }

        if x < 0 {
            return false; 
        } else {
            return x == make_palindrome(0, x);
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = 123;
        let res = Solution::is_palindrome(input);
        assert_eq!(false, res);
    }

    #[test]
    fn test_2() {
        let input = 1;
        let res = Solution::is_palindrome(input);
        assert_eq!(true, res);
    }

    #[test]
    fn test_3() {
        let input = 313;
        let res = Solution::is_palindrome(input);
        assert_eq!(true, res);
    }

    #[test]
    fn test_4() {
        let input = -123;
        let res = Solution::is_palindrome(input);
        assert_eq!(false, res);
    }
}
