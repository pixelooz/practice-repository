use std::collections::HashSet;

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut seen = [0usize; 256];
    let mut left = 0;
    let mut count = 0;
    let s_bytes = s.as_bytes();
    for right in 0..s_bytes.len() {
        while seen[s_bytes[right] as usize] != 0 {
            seen[s_bytes[left] as usize] = 0;
            left += 1;
        }
        seen[s_bytes[right] as usize] = 1;
        count = count.max(right - left + 1);
    }
    count as i32
}
