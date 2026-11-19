pub fn valid_palindrome(s: String) -> bool {
    fn is_pal(s: &[u8], mut l: usize, mut r: usize) -> bool {
        while l < r {
            if s[l] != s[r] {
                return false;
            }
            l += 1;
            r -= 1;
        }
        true
    }
    let s = s.as_bytes();
    let mut l = 0;
    let mut r = s.len() - 1;
    while l < r {
        if s[l] != s[r] {
            return is_pal(s, l + 1, r) || is_pal(s, l, r - 1);
        }
        l += 1;
        r -= 1;
    }
    true
}
