use std::collections::HashSet;

/**
 * TWO NUM SUM
 * Requirements:
 *   - Write a function that takes in a non-empty array of distinct integers and an integer representing a target sum.
 *   - If any two numbers in the input array sum up to the target sum, the function should return them in an array, in any order.
 *   - If no two numbers sum up to the target sum, the function should return an empty array.
 *   - Assume AT MOST ONE PAIR of numbers can sum up to the target.
 **/
fn main() {}

fn two_sum_brute_force(input_array: Vec<i32>, target_sum: i32) -> Vec<i32> {
    // Time complexity: O(n2)
    // Space complexity: O(1)
    let mut result = vec![];
    for (i, num1) in input_array.iter().enumerate() {
        for (j, num2) in input_array[i+1..].iter().enumerate() {
            if num1 + num2 == target_sum {
                result.push(*num1);
                result.push(*num2);
                break;
            }
        }
    }
    result
}

fn two_sum_better(mut input_array: Vec<i32>, target_sum: i32) -> Vec<i32> {
    // Time complexity: O(nlogn)
    // Space complexity: O(1)
    let mut result = vec![];
    input_array.sort();
    let mut left_pointer = 0;
    let mut right_pointer = input_array.len() - 1;
    while right_pointer - left_pointer > 0 {
        if input_array[left_pointer] + input_array[right_pointer] == target_sum {
            result.push(input_array[left_pointer]);
            result.push(input_array[right_pointer]);
            break;
        } else if input_array[left_pointer] + input_array[right_pointer] > target_sum {
            right_pointer -= 1;
        } else {
            left_pointer += 1;
        }
    }
    result
}

fn two_sum_best(mut input_array: Vec<i32>, target_sum: i32) -> Vec<i32> {
    // Time complexity: O(n)
    // Space complexity: O(n)
   let mut result = vec![] ;
   let mut input_set: HashSet<i32> = HashSet::new();
    for input in input_array {
        if input_set.contains(&(target_sum - input)) {
            result.push(input);
            result.push(target_sum - input);
            break;
        }
        input_set.insert(input);
    }
   result
}

#[cfg(test)]
mod s_two_num_sum {
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