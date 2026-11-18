pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
    if nums.is_empty() {
        return vec![];
    }
    let length = nums.len();
    let mut result = vec![0i32; length];
    let mut left = 0;
    let mut right = length - 1;

    for i in (0..length).rev() {
        let left_sq = nums[left].pow(2);
        let right_sq = nums[right].pow(2);
        if left_sq > right_sq {
            result[i] = left_sq;
            left += 1;
        } else {
            result[i] = right_sq;
            right = right.saturating_sub(1);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(sorted_squares(vec![-4, -1, 0, 3, 10]), vec![0, 1, 9, 16, 100]);
    }
}
