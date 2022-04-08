use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn update(sums: &mut [usize], input: File) {
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

fn main() {
    let mut sums = [0usize; 10];
    let files_names = args().into_iter().skip(1);
    let handles = files_names.map(|name| {
        std::thread::spawn(move || {
            let input = File::open(name).unwrap();
            update(&mut sums, input);
        })
    });
    handles.for_each(|h| {
        h.join().unwrap();
    });
    println!("{:?}", sums);
}
