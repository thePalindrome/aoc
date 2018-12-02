fn main() {
    let input = include_str!("input");
    let mut twos = 0;
    let mut threes = 0;
    for line in input.split_whitespace() {
        let mut lettermap = std::collections::HashMap::with_capacity(27);
        for c in line.chars() {
            let count = lettermap.entry(c).or_insert(0);
            *count += 1;
        }
        twos += if lettermap.values().any(|&x| x == 2) {1} else {0};
        threes += if lettermap.values().any(|&x| x == 3) {1} else {0};
    }
    println!("Checksum: {}", twos * threes);

    let mut test = input.split_whitespace().enumerate().cycle(); // Cloned array because otherwise I might make the checker A N G E R Y
    for (i,line) in input.split_whitespace().enumerate() {
        let to_test = test.by_ref().filter(|(x, _y)| x > &i);
        let chars = line.chars();
        let mut index = 0;
        for (fubar,t) in to_test { // Iterate over lines to test
            if fubar < index {
                break;
            }
            else {
                index = fubar;
            }
            let mut fail_count = 0;
            for (c,c2) in t.chars().zip(chars.clone()) { // Iterate over chars in line
                 if c != c2 {
                     fail_count += 1;
                 }
            }
            if fail_count == 1 {
                println!("Result found!\n{}\n{}",line,t);
                return;
            }
        }
    }
}
