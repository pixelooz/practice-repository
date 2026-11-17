pub fn reverse_string(chars: &mut [char]) {
    if chars.is_empty() {
        return;
    }
    let mut i = 0;
    let mut j = chars.len() - 1;
    while i < j {
        let ch_i = chars[i];
        let ch_j = chars[j];
        chars[i] = ch_j;
        chars[j] = ch_i;
        i += 1;
        j -= 1;
    }
}
