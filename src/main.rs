use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::SystemTime;

fn main() -> std::io::Result<()> {
    let f = File::open("data.txt")?;
    let f = BufReader::new(f);
    let mut data: Vec<i64> = vec![0; 10_000_000];

    let start = SystemTime::now();
    f.lines().for_each(|line| {
        let line = line.expect("unable to read line");
        let mut split = line.splitn(3, ' ');
        let start: usize = split
            .next().expect("no start")
            .parse().expect("start was not a i32");
        let end: usize = split
            .next().expect("no end")
            .parse().expect("end was not a i32");
        let val: i64 = split
            .next().expect("no val")
            .parse().expect("val was not a i32");

        data[start] = data[start] + val;
        data[end] = data[end] - val;
    });
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
