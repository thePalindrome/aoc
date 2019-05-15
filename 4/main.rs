use std::collections::HashMap;

struct Guard {
    total_time: u32,
    id: usize,
    minutes: [u32; 60],
}

fn find_max(arr: [u32; 60]) -> usize {
    let mut minute = 0;
    let mut acc = 0;
    for i in 0..60 {
        if arr[i] > acc {
            acc = arr[i];
            minute = i;
        }
    }
    acc as usize
}

fn main() {
    let input: Vec<&str> = include_str!("input").split('\n').collect();
    let mut guards = HashMap::new();
    let mut id: usize = 0;
    let mut sleep: usize = 0;
    for i in input {
        if i.contains("Guard") {
            id = i.split(' ').find(|m| m.starts_with("#")).expect("Bad Input, no #").get(1..).expect("Bad string")
                .parse::<usize>().unwrap(); // Get the guard id in a weird way.
            if !guards.contains_key(&id) {
                guards.insert(id, Guard {total_time: 0, id: id, minutes: [0;60]} );
            }
        } else if i.contains("falls asleep") {
            sleep = i.split(' ').find(|m| m.contains(":")).expect("No sleep time?").get(3..5).expect("bad sleep time")
                .parse::<usize>().unwrap();
        } else if i.contains("wakes up") {
            let wake: usize = i.split(' ').find(|m| m.contains(":")).expect("No wake time?").get(3..5).expect("bad wake time")
                .parse::<usize>().unwrap();
            let guard = guards.get_mut(&id).unwrap();
            guard.total_time += (wake - sleep) as u32;
            for j in sleep..wake {
                guard.minutes[j] += 1;
            }
        } else {
            println!("Bad input: {}",i);
        }
    }
    let sleepyhead = guards.values().max_by(|a, b| a.total_time.cmp(&b.total_time)).unwrap();
    println!("Guard {} slept the longest at {} minutes", sleepyhead.id, sleepyhead.total_time);
    let mut acc = 0;
    let mut minute = 0;
    for i in 0..60 {
        if sleepyhead.minutes[i] > acc {
            acc = sleepyhead.minutes[i];
            minute = i;
        }
    }
    println!("Part one: {}", minute * sleepyhead.id);
    acc = 0;
    minute = 0;
    let dozer = guards.values().max_by(|a, b|
       find_max(a.minutes).cmp(&find_max(b.minutes)) 
    ).unwrap();
    for i in 0..60 {
        if dozer.minutes[i] > acc {
            acc = dozer.minutes[i];
            minute = i;
        }
    }
    println!("Part two: {}", minute * dozer.id);
}
