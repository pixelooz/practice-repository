pub fn backspace_compare(s: String, t: String) -> bool {
    let s_bytes = s.as_bytes();
    let t_bytes = t.as_bytes();
    let mut i = s.len();
    let mut j = t.len();
    while i > 0 || j > 0 {
        if next_valid(s_bytes, &mut i) != next_valid(t_bytes, &mut j) {
            return false;
        }
    }
    true
}

pub fn next_valid(bytes: &[u8], len: &mut usize) -> Option<u8> {
    let mut skips = 0usize;
    while *len > 0 {
        *len -= 1;
        if bytes[*len] == b'#' {
            skips += 1;
            continue;
        }
        if skips > 0 {
            skips -= 1;
            continue;
        }
        return Some(bytes[*len]);
    }
    None
}

#[test]
fn test_backspace_compare() {
    let s = "ab##".into();
    let t = "c#d#".into();
    assert!(backspace_compare(s, t))
}
