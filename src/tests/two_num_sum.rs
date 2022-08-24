pub fn test_two_sum(method: fn(input_array: Vec<i32>, target_sum: i32) -> Vec<i32>) {
    let test_cases = vec![
        (vec![8, 2, 7, 3, 1, 10], 5, vec![2, 3]),
        (vec![21, 4, 2, 30], 20, vec![]),
        (vec![-4, 0, 9, 4, 7, 2], 5, vec![-4, 9]),
        (vec![2], 2, vec![]),
        (vec![232, -2, 1, 18], -1, vec![-2, 1]),
        (vec![1, -1, 0], 0, vec![-1, 1]),
        (vec![23249, 20, 696, 357, 30235, 999, 2333, 50000, 40050, 76750, 45555], 99999, vec![23249, 76750]),
        (vec![0], 10, vec![]),
        (vec![222, 333, 444, 555, 666, 777, 888], 1111, vec![]),
    ];

    for (input_array, target_sum, expected_result) in test_cases {
        let mut result = method(input_array.clone(), target_sum);
        result.sort();
        println!("Testing input array {:?} to find target {}", input_array, target_sum);
        assert_eq!(result, expected_result, "{}", format!("Result {:?} does not match expected_result {:?}", result, expected_result));
    }
}
