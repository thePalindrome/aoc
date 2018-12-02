fn main() {
    let input = include_str!("input").split_whitespace();
    let mut acc: i64 = 0;
    let mut vals:std::collections::HashSet<i64> = std::collections::HashSet::new();
    for line in input.cycle() {
        acc += line.parse::<i64>().unwrap();
        if !vals.insert(acc) {
            panic!("{}",acc);
        }
    }
}
