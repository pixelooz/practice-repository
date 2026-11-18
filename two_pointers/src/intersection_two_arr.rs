pub fn intersection(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32> {
    let mut result = vec![];

    nums1.sort_unstable();
    nums2.sort_unstable();
    let mut i = 0;
    let mut j = 0;

    while i < nums1.len() && j < nums2.len() {
        match nums1[i].cmp(&nums2[j]) {
            std::cmp::Ordering::Less => i += 1,
            std::cmp::Ordering::Equal => {
                if let Some(last) = result.last()
                    && *last == nums1[i]
                {
                    i += 1;
                    j += 1;
                    continue;
                }
                result.push(nums1[i]);
                i += 1;
                j += 1;
            }
            std::cmp::Ordering::Greater => j += 1,
        }
    }
    result
}
