use std::cmp::Ordering;

pub fn trap(heights: Vec<i32>) -> i32 {
    if heights.len() < 3 {
        return 0;
    }
    let (mut left, mut right) = (0, heights.len() - 1);
    let mut left_max = heights[left];
    let mut right_max = heights[right];
    let mut water = 0;
    while left < right {
        match left_max.cmp(&right_max) {
            Ordering::Less => {
                left += 1;
                left_max = left_max.max(heights[left]);
                water += left_max - heights[left];
            }
            _ => {
                right -= 1;
                right_max = right_max.max(heights[right]);
                water += right_max - heights[right];
            }
        }
    }
    water
}
