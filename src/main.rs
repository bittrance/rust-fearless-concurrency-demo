use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::AddAssign;
use std::sync::{Arc, Mutex};

#[derive(Debug, Default)]
struct MyUsize(usize);
impl AddAssign for MyUsize {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

fn update(sums: &mut Arc<[Mutex<MyUsize>; 10]>, input: File) {
    let reader = BufReader::new(input);
    for line in reader.lines().map(Result::unwrap) {
        if let Some((key_str, val_str)) = line.split_once(',') {
            let key: usize = key_str.parse().unwrap();
            let val: usize = val_str.parse().unwrap();
            *sums[key].lock().unwrap() += MyUsize(val);
        } else {
            panic!("Bad line: {}", line);
        }
    }
}

fn main() {
    let sums: Arc<[Mutex<MyUsize>; 10]> = Default::default();
    let files_names = args().into_iter().skip(1);
    let handles = files_names.map(|name| {
        let mut sums = Arc::clone(&sums);
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
