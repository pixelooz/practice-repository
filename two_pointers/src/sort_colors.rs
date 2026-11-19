pub fn sort_colors(nums: &mut [i32]) {
    let mut low = 0;
    let mut mid = 0;
    let mut high = nums.len() - 1;
    while mid <= high {
        match nums[mid] {
            0 => {
                nums.swap(low, mid);
                low += 1;
                mid += 1;
            }
            2 => {
                nums.swap(high, mid);
                high -= 1;
            }
            _ => mid += 1,
        }
    }
}
