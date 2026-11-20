use std::cmp::Ordering;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let (mut low, mut high) = (0, nums.len() - 1);
    while low < high {
        let sum = nums[low] + nums[high];
        match sum.cmp(&target) {
            Ordering::Greater => high -= 1,
            Ordering::Less => low += 1,
            Ordering::Equal => {
                let low = i32::try_from(low + 1).unwrap();
                let high = i32::try_from(high + 1).unwrap();
                return vec![low, high];
            }
        }
    }
    vec![]
}

#[test]
fn test_two_sum() {
    let arr = vec![2, 7, 11, 15];
    let res = two_sum(arr, 9);
    assert_eq!(res, vec![1, 2])
}
