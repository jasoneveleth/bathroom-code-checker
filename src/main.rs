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

#[allow(unused)]
fn repack(s: (u8, u8, u8, u8)) -> u32 {
    (s.3 as u32)*1000 + (s.2 as u32)*100 + (s.1 as u32)*10 + (s.0 as u32)
}

fn gen() -> Result<Vec<u8>, String> {
    let mut nums_left: Vec<i32> = (1..=10_000).collect();
    let mut rng = thread_rng();
    nums_left.shuffle(&mut rng);

    let first = nums_left.pop().unwrap();
    let (thousand, hundred, ten, one) = unpack(first as u32);
    let mut s = vec![thousand, hundred, ten, one];

    while nums_left.len() > 0 {
        let (goal_thousand, goal_hundred, goal_ten) = (s[s.len()-3], s[s.len()-2], s[s.len()-1]);
        let mut arg_max = nums_left.len();
        for (i, num) in nums_left.iter().enumerate().rev() {
            let (thousand, hundred, ten, _) = unpack(*num as u32);
            if thousand == goal_thousand && hundred == goal_hundred && ten == goal_ten {
                arg_max = i;
                break;
            }
        }
        if arg_max == nums_left.len() {
            return Err("no more numbers to add".to_string());
        }
        let num = nums_left.remove(arg_max);
        let (_, _, _, one) = unpack(num as u32);
        s.push(one);
    }
    Ok(s)
}

fn main() {
    let filename = "/tmp/c.txt";

    let mut s = gen();
    while !s.is_ok() {
        s = gen();
    }
    let s = s.unwrap();

    println!("found! length {}, written to {}", s.len(), filename);
    std::fs::write(filename, to_s(&s)).unwrap();
}
