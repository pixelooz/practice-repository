use std::cmp::Ordering;

pub fn find_pairs(mut nums: Vec<i32>, k: i32) -> i32 {
    let Ok(k) = u32::try_from(k) else {
        return 0;
    };
    nums.sort_unstable();

    let (mut low, mut high) = (0, 1);
    let mut pairs = 0;

    while high < nums.len() {
        match nums[low].abs_diff(nums[high]).cmp(&k) {
            Ordering::Less => high += 1,
            Ordering::Greater => {
                low += 1;
                if low == high {
                    high += 1;
                }
            }
            Ordering::Equal => {
                pairs += 1;
                low += 1;
                while low < nums.len() && nums[low] == nums[low - 1] {
                    low += 1;
                }
                high = high.max(low + 1);
            }
        }
    }
    pairs
}

#[test]
fn test_find_pairs() {
    let mut arr = vec![1, 3, 1, 5, 4];
    let res = find_pairs(arr, 0);
    assert_eq!(res, 1);
}
