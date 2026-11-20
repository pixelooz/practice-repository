use std::cmp::Ordering;

pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
    let mut closest = nums[0] + nums[1] + nums[2];
    let len = nums.len();
    nums.sort_unstable();

    for i in 0..len - 2 {
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }
        let mut low = i + 1;
        let mut high = len - 1;
        while low < high {
            let sum = nums[i] + nums[low] + nums[high];
            if sum.abs_diff(target) < closest.abs_diff(target) {
                closest = sum;
            }
            match sum.cmp(&target) {
                Ordering::Less => low += 1,
                Ordering::Equal => return sum,
                Ordering::Greater => high -= 1,
            }
        }
    }
    closest
}

// my version
pub fn _three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
    let mut smallest = i32::MAX;
    let mut closest = i32::MAX;
    let mut set_closest = |sum: i32| {
        let diff = (target - sum).abs();
        if diff < smallest {
            smallest = diff;
            closest = sum;
        }
    };
    nums.sort_unstable();
    for i in 0..nums.len() - 2 {
        let mut low = i + 1;
        let mut high = nums.len() - 1;
        while low < high {
            let sum = nums[i] + nums[low] + nums[high];
            match sum.cmp(&target) {
                Ordering::Less => {
                    set_closest(sum);
                    low += 1;
                }
                Ordering::Equal => return sum,
                Ordering::Greater => {
                    set_closest(sum);
                    high -= 1;
                }
            }
        }
    }
    closest
}

#[test]
fn test_three_sum_closest() {
    let mut arr = vec![-1, 2, 1, -4];
    assert_eq!(three_sum_closest(arr, 1), 2);
}
