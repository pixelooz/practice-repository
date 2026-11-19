pub fn remove_element(nums: &mut [i32], val: i32) -> i32 {
    let mut write = 0;
    for read in 0..nums.len() {
        if nums[read] != val {
            nums.swap(write, read);
            write += 1;
            continue;
        }
    }
    i32::try_from(write).unwrap()
}
