pub mod tests;

use clap::Parser;
use sha256::digest;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{mpsc, Arc, Barrier};
use std::thread;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of zeros to find at the end of hash
    #[arg(short = 'N', long)]
    n: usize,

    /// Number of hashes to find
    #[arg(short = 'F', long)]
    find: usize,
}

fn main() {
    let args = Args::parse();
    let n = args.n;
    let find = args.find;
    let result = find_n_0_hash(n, find);
    for i in result {
        println!("{}, \"{}\"", i.0, i.1);
    }
}

/// Calculates the sha256 hash, and outputs the hash and the original number
/// to the console if the hash digest ends with N characters zero.
/// The F parameter defines how many hash values the command should find.
///
/// # Panics:
/// If N is 0.
pub fn find_n_0_hash(n: usize, find: usize) -> Vec<(usize, std::string::String)> {
    if n == 0 {
        panic!("N of zeros must be at least 1");
    }
    // To keep track of maximum value thread reached
    let max_step = Arc::new(AtomicUsize::new(0));
    let counter = Arc::new(AtomicUsize::new(0));

    let (tx, rx) = mpsc::channel();
    let cpus = num_cpus::get();
    let mut handles = Vec::new();
    let barrier = Arc::new(Barrier::new(cpus));

    // Start spawning threads
    for i in 0..cpus {
        let c = counter.clone();
        let max = max_step.clone();
        let bar = barrier.clone();
        let filter = "0".repeat(n);
        let thread_tx = tx.clone();
        // Collect local results of each thread
        let mut local_queue = Vec::new();
        let handle = thread::spawn(move || {
            // Calculate hash, each thread starts with index of its own number and goes by step equal to amount of cpus.
            for mut v in (i..usize::MAX).step_by(cpus) {
                if c.load(Ordering::SeqCst) < find {
                    let hash = digest(v.to_string());
                    if hash.ends_with(&filter) {
                        // Found hash => increment atomic counter.
                        c.fetch_add(1, Ordering::SeqCst);
                        local_queue.push((v, hash));
                    }
                // Got enough hashes, let every thread to calculate hashes until max step,
                // possibly insert new hashes that are lower value.
                } else {
                    let mut current_max = max.load(Ordering::SeqCst);
                    while current_max < v {
                        match max.compare_exchange_weak(
                            current_max,
                            v,
                            Ordering::SeqCst,
                            Ordering::SeqCst,
                        ) {
                            Ok(_) => break,
                            Err(new_max) => {
                                if new_max > current_max {
                                    current_max = new_max;
                                }
                            }
                        }
                    }
                    // Barrier allows every thread to catch up and get true current_max
                    bar.wait();

                    while v < max.load(Ordering::SeqCst) {
                        let hash = digest(v.to_string());
                        if hash.ends_with(&filter) {
                            local_queue.push((v, hash));
                        }
                        v += cpus;
                    }
                    thread_tx.send(local_queue).unwrap();
                    break;
                }
            }
        });
        handles.push(handle);
    }

    let mut results = Vec::with_capacity(cpus);
    for _ in 0..cpus {
        results.push(rx.recv());
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let mut result = Vec::new();
    for i in results {
        result.append(&mut i.unwrap());
    }
    result.sort();
    return result[0..find].to_vec();
}
