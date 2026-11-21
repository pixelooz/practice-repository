pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
    let Ok(k) = usize::try_from(k) else {
        return false;
    };
    let mut set = std::collections::HashSet::new();

    for i in 0..nums.len() {
        if !set.insert(nums[i]) {
            return true;
        }
        if set.len() > k {
            set.remove(&nums[i - k - 1]);
        }
    }
    false
}

#[test]
fn test_contains_nearby_duplicate() {
    let arr = vec![1, 2, 3, 1, 2, 3];
    let res = contains_nearby_duplicate(arr, 2);
    assert!(res);
}
