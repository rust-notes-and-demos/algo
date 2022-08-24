/**
 * TWO NUM SUM
 * Requirements:
 *   - Write a function that takes in a non-empty array of distinct integers and an integer representing a target sum.
 *   - If any two numbers in the input array sum up to the target sum, the function should return them in an array, in any order.
 *   - If no two numbers sum up to the target sum, the function should return an empty array.
 *   - Assume AT MOST ONE PAIR of numbers can sum up to the target.
 **/

fn two_sum_brute_force(input_array: Vec<i32>, target_sum: i32) -> Vec<i32> {
    // Time complexity: O(n2)
    // Space complexity: O(1)
    todo!()
}

fn two_sum_better(input_array: Vec<i32>, target_sum: i32) -> Vec<i32> {
    // Time complexity: O(nlogn)
    // Space complexity: O(1)
    todo!()
}

fn two_sum_best(input_array: Vec<i32>, target_sum: i32) -> Vec<i32> {
    // Time complexity: O(n)
    // Space complexity: O(n)
    todo!()
}

#[cfg(test)]
mod q_two_num_sum {
    use super::*;
    use crate::tests::two_num_sum::test_two_sum;

    #[test]
    fn test_two_sum_brute_force() {
        test_two_sum(two_sum_brute_force);
    }

    #[test]
    fn test_two_sum_better() {
        test_two_sum(two_sum_better);
    }

    #[test]
    fn test_two_sum_best() {
        test_two_sum(two_sum_best);
    }
}