use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut sums = [0usize; 10];
    let files_names = ["file-1", "file-2"];
    for name in files_names {
        let input = File::open(name).unwrap();
        let reader = BufReader::new(input);
        for row in reader.lines().map(Result::unwrap) {
            if let Some((key_str, val_str)) = row.split_once(',') {
                let key: usize = key_str.parse().unwrap();
                let val: usize = val_str.parse().unwrap();
                sums[key] += val;
            } else {
                panic!("Bad line: {}", row);
            }
        }
    }
    println!("{:?}", sums);
}
