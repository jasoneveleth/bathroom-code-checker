use std::sync::{Arc, RwLock};
use rand::{thread_rng, Rng};
use std::thread;
use rand::seq::SliceRandom;

fn to_s(s: &[u8]) -> String {
    // val to ascii
    let vals = s.iter().map(|&x| x + 48).collect::<Vec<u8>>();
    String::from_utf8(vals).unwrap()
}

fn unpack(s: u32) -> (u8, u8, u8, u8) {
    let one = s % 10;
    let ten = (s / 10) % 10;
    let hundred = (s / 100) % 10;
    let thousand = (s / 1000) % 10;
    (thousand as u8, hundred as u8, ten as u8, one as u8)
}

fn repack(s: (u8, u8, u8, u8)) -> u32 {
    (s.3 as u32)*1000 + (s.2 as u32)*100 + (s.1 as u32)*10 + (s.0 as u32)
}

fn check(s: &[u8]) -> bool {
    let mut seen = [0u8; 10000];
    let mut val = repack((s[0], s[1], s[2], s[3]));
    seen[val as usize] = 1;

    for i in 4..s.len() {
        val = (val % 1000) * 10 + s[i] as u32;
        seen[val as usize] = 1;
    }

    seen.iter().all(|&x| x == 1)
}

fn loud_check(s: &[u8]) {
    let mut seen = [0u8; 10000];
    let mut val = repack((s[0], s[1], s[2], s[3]));
    seen[val as usize] = 1;

    for i in 4..s.len() {
        val = (val % 1000) * 10 + s[i] as u32;
        seen[val as usize] = 1;
    }

    for i in 0..seen.len() {
        if seen[i] == 0 {
            println!("missing: {:?}", unpack(i as u32));
        }
    };
}

fn writer(shared: Arc<RwLock<Vec<u8>>>) {
    let mut rng = thread_rng();
    let mut bestlen = 40_000;

    loop {
        let mut s = gen();
        let mut failcount = 0;
        while failcount < 1000 {
            let random_index = rng.gen_range(0..s.len());
            let mut new = s.clone();
            new.remove(random_index);

            if check(&new) {
                if bestlen > new.len() {
                    let mut data = shared.write().unwrap();
                    *data = new.clone();
                    bestlen = new.len();
                }
                s = new;
                failcount = 0;
            } else {
                failcount += 1;
            }
        }
    }
}

fn writer2(shared: Arc<RwLock<Vec<u8>>>) {
    let mut bestlen = 40_000;

    loop {
        let new = gen_liam();
        if check(&new) {
            if bestlen > new.len() {
                let mut data = shared.write().unwrap();
                *data = new.clone();
                bestlen = new.len();
            }
        }
    }
}

fn reader(shared: Arc<RwLock<Vec<u8>>>) {
    loop {
        let s = shared.read().unwrap();
        println!("Found len: {}, see /tmp/a.txt", s.len());
        // write value to file
        std::fs::write("/tmp/a.txt", to_s(&s)).unwrap();
        thread::sleep(std::time::Duration::from_secs(5));
    }
}

fn gen_liam() -> Vec<u8> {
    let mut range: Vec<i32> = (1..=10_000).collect();
    let mut rng = thread_rng();
    range.shuffle(&mut rng);

    let first = range.pop().unwrap();
    let (thousand, hundred, ten, one) = unpack(first as u32);
    let mut s = vec![thousand, hundred, ten, one];

    while range.len() > 0 {
        let (goal_thousand, goal_hundred, goal_ten) = (s[s.len()-3], s[s.len()-2], s[s.len()-1]);
        let mut index = range.len() - 1;
        let mut best_overlap = 0;
        for (i, num) in range.iter().enumerate().rev() {
            let (thousand, hundred, ten, _) = unpack(*num as u32);
            let overlap = if thousand == goal_thousand { 1 } else { 0 } +
                         if hundred == goal_hundred { 1 } else { 0 } +
                         if ten == goal_ten { 1 } else { 0 };
            if overlap > best_overlap {
                best_overlap = overlap;
                index = i;
            }
        }
        let num = range.remove(index);
        let (thousand, hundred, ten, one) = unpack(num as u32);
        if best_overlap == 3 {
            s.push(one);
        } else if best_overlap == 2 {
            s.push(ten);
            s.push(one);
        } else if best_overlap == 1 {
            s.push(hundred);
            s.push(ten);
            s.push(one);
        } else {
            s.push(thousand);
            s.push(hundred);
            s.push(ten);
            s.push(one);
        }
    }
    s
}

fn gen() -> Vec<u8> {
    let mut range: Vec<i32> = (1..=10_000).collect();
    let mut rng = thread_rng();
    range.shuffle(&mut rng);

    let mut s = vec![];

    for i in range.iter() {
        let one = i % 10;
        let ten = (i / 10) % 10;
        let hundred = (i / 100) % 10;
        let thousand = (i / 1000) % 10;
        s.push(one as u8);
        s.push(ten as u8);
        s.push(hundred as u8);
        s.push(thousand as u8);
    }
    s
}

fn main() {
    // let s = gen();
    // let swappable = Arc::new(RwLock::new(s));

    let mut s = gen_liam();
    while s.len() != 10_003 {
        s = gen_liam();
    }
    println!("len {}", s.len());
    std::fs::write("/tmp/c.txt", to_s(&s)).unwrap();

    // thread::scope(|s| {
    //     s.spawn(|| writer2(swappable.clone()));
    //     s.spawn(|| reader(swappable.clone()));
    // });
}
