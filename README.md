# Rust fearless concurrency demo

One of the goals of Rust is to promote ["fearless concurrency"](https://doc.rust-lang.org/book/ch16-00-concurrency.html). This repository demonstrates how Rust's memory safety guarantees underpin this goal. With creative use of the type system, many risky patterns become compile-time errors. The repository is a series of commits starting with a single-threaded program to read and sum numbers from files. Each commit takes a step towards a fully multi-threaded version, with its commit message trying to explain what is happening. Some commits contain errors that shows how the Rust compiler refuses to compile code that risks concurrency issues.

## Results of the commits

This shows the output of running `cargo run -- file-*` on each commit in turn.

### Switched to 92d8e4a

```
   Compiling rust-memory-safety v0.1.0 (/home/bittrance/projects/personal/rust-memory-safety)
    Finished dev [unoptimized + debuginfo] target(s) in 0.21s
     Running `target/debug/rust-memory-safety file-1 file-2 file-3 file-4 file-5 file-6 file-7 file-8 file-9`
[MyUsize(35396), MyUsize(34665), MyUsize(34714), MyUsize(34643), MyUsize(35710), MyUsize(35074), MyUsize(35095), MyUsize(34765), MyUsize(34104), MyUsize(35218)]
```

### Switched to d888b1d

```
   Compiling rust-memory-safety v0.1.0 (/home/bittrance/projects/personal/rust-memory-safety)
    Finished dev [unoptimized + debuginfo] target(s) in 0.22s
     Running `target/debug/rust-memory-safety file-1 file-2 file-3 file-4 file-5 file-6 file-7 file-8 file-9`
[MyUsize(35396), MyUsize(34665), MyUsize(34714), MyUsize(34643), MyUsize(35710), MyUsize(35074), MyUsize(35095), MyUsize(34765), MyUsize(34104), MyUsize(35218)]
```

### Switched to 404765a

```
   Compiling rust-memory-safety v0.1.0 (/home/bittrance/projects/personal/rust-memory-safety)
error: lifetime may not live long enough
  --> src/main.rs:31:9
   |
30 |       let handles = files_names.map(|name| {
   |                                     ------ lifetime `'1` represents this closure's body
31 | /         std::thread::spawn(|| {
32 | |             let input = File::open(name).unwrap();
33 | |             update(&mut sums, input);
34 | |         })
   | |__________^ argument requires that `'1` must outlive `'static`
   |
   = note: closure implements `FnMut`, so references to captured variables can't escape the closure

error[E0597]: `sums` does not live long enough
  --> src/main.rs:33:25
   |
30 |       let handles = files_names.map(|name| {
   |                                     ------ value captured here
31 | /         std::thread::spawn(|| {
32 | |             let input = File::open(name).unwrap();
33 | |             update(&mut sums, input);
   | |                         ^^^^ borrowed value does not live long enough
34 | |         })
   | |__________- argument requires that `sums` is borrowed for `'static`
...
40 |   }
   |   - `sums` dropped here while still borrowed

error[E0502]: cannot borrow `sums` as immutable because it is also borrowed as mutable
  --> src/main.rs:39:22
   |
30 |       let handles = files_names.map(|name| {
   |                                     ------ mutable borrow occurs here
31 | /         std::thread::spawn(|| {
32 | |             let input = File::open(name).unwrap();
33 | |             update(&mut sums, input);
   | |                         ---- first borrow occurs due to use of `sums` in closure
34 | |         })
   | |__________- argument requires that `sums` is borrowed for `'static`
...
39 |       println!("{:?}", sums);
   |                        ^^^^ immutable borrow occurs here
   |
   = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)

Some errors have detailed explanations: E0502, E0597.
For more information about an error, try `rustc --explain E0502`.
error: could not compile `rust-memory-safety` due to 3 previous errors
```

### Switched to 0b7f22b

```
   Compiling rust-memory-safety v0.1.0 (/home/bittrance/projects/personal/rust-memory-safety)
error[E0507]: cannot move out of `sums`, a captured variable in an `FnMut` closure
  --> src/main.rs:31:28
   |
28 |       let mut sums: [MyUsize; 10] = Default::default();
   |           -------- captured outer variable
29 |       let files_names = args().into_iter().skip(1);
30 |       let handles = files_names.map(|name| {
   |  ___________________________________-
31 | |         std::thread::spawn(move || {
   | |                            ^^^^^^^ move out of `sums` occurs here
32 | |             let input = File::open(name).unwrap();
33 | |             update(&mut sums, input);
   | |                         ----
   | |                         |
   | |                         move occurs because `sums` has type `[MyUsize; 10]`, which does not implement the `Copy` trait
   | |                         move occurs due to use in closure
34 | |         })
35 | |     });
   | |_____- captured by this `FnMut` closure

error[E0382]: borrow of moved value: `sums`
  --> src/main.rs:39:22
   |
28 |     let mut sums: [MyUsize; 10] = Default::default();
   |         -------- move occurs because `sums` has type `[MyUsize; 10]`, which does not implement the `Copy` trait
29 |     let files_names = args().into_iter().skip(1);
30 |     let handles = files_names.map(|name| {
   |                                   ------ value moved into closure here
...
33 |             update(&mut sums, input);
   |                         ---- variable moved due to use in closure
...
39 |     println!("{:?}", sums);
   |                      ^^^^ value borrowed here after move
   |
   = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)

Some errors have detailed explanations: E0382, E0507.
For more information about an error, try `rustc --explain E0382`.
error: could not compile `rust-memory-safety` due to 2 previous errors
```

### Switched to b093a78

```
   Compiling rust-memory-safety v0.1.0 (/home/bittrance/projects/personal/rust-memory-safety)
error[E0596]: cannot borrow data in an `Arc` as mutable
  --> src/main.rs:21:13
   |
21 |             sums[key] += MyUsize(val);
   |             ^^^^^^^^^ cannot borrow as mutable
   |
   = help: trait `DerefMut` is required to modify through a dereference, but it is not implemented for `Arc<[MyUsize; 10]>`

For more information about this error, try `rustc --explain E0596`.
error: could not compile `rust-memory-safety` due to previous error
```

### Switched to b093a78

```
   Compiling rust-memory-safety v0.1.0 (/home/bittrance/projects/personal/rust-memory-safety)
error[E0596]: cannot borrow data in an `Arc` as mutable
  --> src/main.rs:21:13
   |
21 |             sums[key] += MyUsize(val);
   |             ^^^^^^^^^ cannot borrow as mutable
   |
   = help: trait `DerefMut` is required to modify through a dereference, but it is not implemented for `Arc<[MyUsize; 10]>`

For more information about this error, try `rustc --explain E0596`.
error: could not compile `rust-memory-safety` due to previous error
```

### Switched to b1fd430

```
   Compiling rust-memory-safety v0.1.0 (/home/bittrance/projects/personal/rust-memory-safety)
    Finished dev [unoptimized + debuginfo] target(s) in 0.25s
     Running `target/debug/rust-memory-safety file-1 file-2 file-3 file-4 file-5 file-6 file-7 file-8 file-9`
[Mutex { data: MyUsize(35396), poisoned: false, .. }, Mutex { data: MyUsize(34665), poisoned: false, .. }, Mutex { data: MyUsize(34714), poisoned: false, .. }, Mutex { data: MyUsize(34643), poisoned: false, .. }, Mutex { data: MyUsize(35710), poisoned: false, .. }, Mutex { data: MyUsize(35074), poisoned: false, .. }, Mutex { data: MyUsize(35095), poisoned: false, .. }, Mutex { data: MyUsize(34765), poisoned: false, .. }, Mutex { data: MyUsize(34104), poisoned: false, .. }, Mutex { data: MyUsize(35218), poisoned: false, .. }]
```
