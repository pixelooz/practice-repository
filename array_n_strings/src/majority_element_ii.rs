pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
    let len = nums.len();
    let mut count_map = std::collections::HashMap::new();
    for num in nums {
        *count_map.entry(num).or_insert(0usize) += 1;
    }
    count_map.into_iter().filter_map(|(key, value)| (value > len / 3).then_some(key)).collect()
}
