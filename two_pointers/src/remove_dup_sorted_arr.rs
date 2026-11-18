pub fn remove_duplicates(nums: &mut [i32]) -> i32 {
    if nums.is_empty() {
        return 0;
    }
    let mut write = 1;
    for read in 1..nums.len() {
        if nums[write - 1] != nums[read] {
            nums[write] = nums[read];
            write += 1;
        }
    }
    i32::try_from(write).unwrap()
}
