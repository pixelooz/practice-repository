pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
    let mut map = std::collections::HashMap::new();
    let mut count = 0;
    map.insert(0, 1);
    let mut sum = 0;
    for num in nums {
        sum += num;
        if let Some(value) = map.get(&(sum - k)) {
            count += value;
        }
        map.entry(sum).or_insert(1);
    }
    count
}
