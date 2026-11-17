pub fn is_subsequence(s: String, t: String) -> bool {
    if s.len() > t.len() {
        return false;
    }

    let s = s.as_bytes();
    let t = t.as_bytes();
    let mut i = 0;
    let mut j = 0;
    while i < s.len() && j < t.len() {
        if s[i] == t[j] {
            i += 1;
            j += 1;
        } else {
            j += 1;
        }
        if i > j {
            return false;
        }
    }
    !(i < s.len() && j >= t.len())
}

#[test]
fn test_is_subsequence() {
    let s = "abc".into();
    let t = "asdfbkku".into();
    let res = is_subsequence(s, t);
    assert!(!res, "should be not true");

    let s = "abc".into();
    let t = "asdfbkkuecc".into();
    let res = is_subsequence(s, t);
    assert!(res, "should true");
}
