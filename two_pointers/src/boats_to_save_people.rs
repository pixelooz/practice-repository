pub fn num_rescue_boats(mut people: Vec<i32>, limit: i32) -> i32 {
    let mut boats = 0;
    let mut low = 0;
    let mut high = people.len();
    people.sort_unstable();
    while low < high {
        high -= 1;
        if low < high && people[low] + people[high] < limit {
            low += 1;
        }
        boats += 1;
    }
    boats
}

#[test]
fn test_num_rescue_boats() {
    let people = vec![1, 2];
    let res = num_rescue_boats(people, 3);
    assert_eq!(res, 1);
}
