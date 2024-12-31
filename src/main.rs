use rand::thread_rng;
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

fn main() {
    let mut s = gen_liam();
    while s.len() != 10_003 {
        s = gen_liam();
    }
    println!("len {}", s.len());
    std::fs::write("/tmp/c.txt", to_s(&s)).unwrap();
}
