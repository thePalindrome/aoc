#[derive(Debug)]
struct Claim {
    top: usize,
    left: usize,
    width: usize,
    height: usize,
}

fn main() {
    let mut input = include_str!("input").split('\n');
    let mut claims: Vec<Claim> = Vec::with_capacity(1267*4);
    let _ = input.next_back();
    for i in input {
        let mut iter = i.split(" ");
        let _ = { iter.next() }; // Delete the claim number
        let _ = { iter.next() }; // Delete the @
        let mut lt = { iter.next().expect("lt").split(",") }; // Get the top and left
        let mut wt = iter.next().expect("wt").split("x"); // Get the width an height
        claims.push(Claim {
            left: lt.next().expect("l").parse().unwrap(),
            top: lt.next().expect("t").split(":").nth(0).expect("").parse().unwrap(),
            width: wt.next().expect("").parse().unwrap(),
            height: wt.next().expect("").parse().unwrap()
        });
    }
    let mut grid: [[u8; 1500]; 1500] = [[0; 1500]; 1500];
    for i in &claims {
        for x in i.top..i.top+i.height {
            for y in i.left..i.left+i.width {
                grid[y][x] += 1;
            }
        }
    }
    let mut acc = 0;
    for y in 0..1000 {
        for x in 0..1000 {
            if grid[y][x] > 1 {
                acc +=1;
            }
        }
    }
    println!("{} square inches are contested", acc);

    for (i,c) in claims.iter().enumerate() {
        let mut contested = false;

        for x in c.top..c.top+c.height {
            for y in c.left..c.left+c.width {
                if grid[y][x] > 1 {
                    contested = true;
                }
            }
        }

        if contested == false {
            println!("claim #{} is not contested", i + 1);
            return;
        }
    }
}
