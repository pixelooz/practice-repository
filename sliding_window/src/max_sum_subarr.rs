pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
    let Ok(k) = usize::try_from(k) else {
        return 0.0;
    };
    if k == 0 || k > nums.len() {
        return 0.0;
    }
    let mut window_sum: i64 = nums[..k].iter().map(|&x| i64::from(x)).sum();
    let mut sum = window_sum;
    for i in k..nums.len() {
        window_sum += i64::from(nums[i] - nums[i - k]);
        sum = sum.max(window_sum);
    }
    sum as f64 / k as f64
}
