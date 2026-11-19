pub fn compress(chars: &mut [char]) -> i32 {
    let mut write = 0;
    let mut read = 0;

    while read < chars.len() {
        let current = chars[read];
        let start = read;
        while read < chars.len() && current == chars[read] {
            read += 1;
        }
        chars[write] = current;
        write += 1;
        let count = read - start;
        if count > 1 {
            for ch in count.to_string().chars() {
                chars[write] = ch;
                write += 1;
            }
        }
    }
    i32::try_from(write).unwrap()
}

#[test]
fn test_compress() {
    let mut arr = vec!['a', 'a', 'b', 'b', 'c', 'c', 'c'];
    let res = compress(&mut arr);
    assert_eq!(res, 6);
}
