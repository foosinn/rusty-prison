// stefan@xps ~/git/foosinn/rusty-prison master* 
// $ cargo build --release && ./target/release/rusty-prison
//     Finished release [optimized] target(s) in 0.00s
// load time: 311.537535ms
// result: 585778044
// run time: 49.542291ms

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::SystemTime;

fn main() -> std::io::Result<()> {
    let f = File::open("data.txt")?;
    let mut f = BufReader::new(f);
    let mut data: Vec<i64> = vec![0; 10_000_000];

    let start = SystemTime::now();
    let mut buf = vec![];
    while f.read_until(b'\n', &mut buf).unwrap_or(0) > 0 {
        let mut split = buf.splitn(3, |b| *b == b' ');
        let start: usize = split
            .next().expect("no start")
            .iter().fold(0, |sum, digit| (sum * 10) + (*digit as usize - 48));
        let end: usize = split
            .next().expect("no end")
            .iter().fold(0, |sum, digit| (sum * 10) + (*digit as usize - 48));
        let val: i64 = split.next().expect("no val")[0] as i64 - 48;

        data[start] = data[start] + val;
        data[end] = data[end] - val;
        buf.clear()
    }
    println!("load time: {:?}", start.elapsed().unwrap());

    let start = SystemTime::now();
    let mut delta: i64 = 0;
    let result = data.iter().fold(1 as i64, |product, item| {
        delta += *item;
        let step_delta = delta % 10;
        if step_delta == 0 {
            return product;
        }

        (product * step_delta) % 999999937
    });
    println!("result: {}", result);
    println!("run time: {:?}", start.elapsed().unwrap());

    Ok(())
}
